use std::collections::HashSet;

/* Modules */
use crate::mlflow::{self, config::project::ProjectConfiguration};
use log::{debug, info, warn};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    /* Load Configuration */
    let config = ProjectConfiguration::load()?;
    /* Read all models in this deployment repo */
    let models = config.list_models()?;
    /* Get a list of all Existing Models in the Registry */
    let mut existing_projects: Vec<String> = mlflow::model_registry::list()?
        .iter()
        .map(|x| x.name.clone())
        .filter(|x| x.starts_with(&format!("{}-", config.prefix)))
        .collect();
    /* Find abandoned models & create new models */
    for model in &models {
        let expected_name = format!("{}-{}", config.prefix, model.label);
        if let Some(i) = existing_projects.iter().position(|x| x == &expected_name) {
            /* Case a matching model has been found */
            debug!(
                "Found matching Model Registry for '{}'",
                existing_projects.remove(i)
            );
            continue;
        }
        /* Create a new Model since this one is missing */
        let mut tags = HashSet::new();
        tags.insert(config.tag.clone());
        tags.extend(model.tags.clone());
        mlflow::model_registry::create(
            &expected_name,
            Some(model.description.clone()),
            Some(tags.iter().map(|x| x.clone()).collect()),
        )?;
        info!(
            "Successfully created a new Model Registry entry for '{}'",
            expected_name
        );
    }
    /* Warn about models in the registry that shouldn't be there anymore */
    for x in &existing_projects {
        warn!("The model '{}' is present in the Registry but there is no corresponding model in this repository", x);
    }
    Ok(())
}
