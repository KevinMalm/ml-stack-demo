/* Imports */
use log::{debug, error, info};
use std::path::PathBuf;

/* Local Imports */
use crate::{constants, mlflow::config::model::ModelConfiguration, template, util};

pub fn main(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = ModelConfiguration::new(model);

    /* Create the model source code folder */
    let model_src_dir = PathBuf::from(constants::configuration::env_home())
        .join(constants::configuration::MODELS_FOLDER)
        .join(config.to_folder_name());
    debug!("Creating model folder {}", model_src_dir.display());

    if util::file::create_dir(&model_src_dir) == false {
        error!("Failed to create directory {}", model_src_dir.display());
        return Err("File IO Error".into());
    }

    write_default_files(&config, &model_src_dir)?;

    config.save()?;
    info!("Successfully added a new project model for {}", model);
    return Ok(());
}

fn write_default_files(
    config: &ModelConfiguration,
    home: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    /* Write Default Files */
    for (path, content) in vec![
        (template::MAIN_INVOKE_PY_FILE_NAME, template::MAIN_INVOKE_PY),
        (template::MAIN_TEST_PY_FILE_NAME, template::MAIN_TEST_PY),
        (template::MAIN_TRAIN_PY_FILE_NAME, template::MAIN_TRAIN_PY),
    ] {
        let path = PathBuf::from(&home).join(path);
        let content = content
            .replace(constants::placeholders::PROJECT_NAME, &config.label)
            .to_string();

        debug!("Initializing default file {}", path.display());
        if util::file::write_content(&path, &content) == false {
            error!("Failed to write into {}", path.display());
            return Err("File IO Error".into());
        }
    }

    Ok(())
}
