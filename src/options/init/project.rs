/* Modules */
use log::{debug, error, info};
use std::path::PathBuf;

use crate::{constants, mlflow::config::project::ProjectConfiguration, template, util};

pub fn main(project: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = ProjectConfiguration::new(project);
    let home = util::formatting::format_folder(project);

    create_directories(&home)?;
    write_default_files(&config, &home)?;

    config
        .save_to(PathBuf::from(home).join(constants::configuration::PROJECT_CONFIGURATION_FILE))?;
    info!(
        "Successfully initialized a new project folder for {}",
        project
    );
    Ok(())
}

fn create_directories(home: &str) -> Result<(), Box<dyn std::error::Error>> {
    /* Create Directories */
    for dir in vec![
        constants::configuration::MODELS_FOLDER,
        constants::configuration::PROJECT_HOME_MODEL_CONFIG_FOLDER,
        constants::configuration::TEMPLATES_CONFIG_FOLDER,
        constants::configuration::TERRAFORM_FOLDER,
    ] {
        let path = PathBuf::from(&home).join(dir);
        debug!("Creating project folder {}", path.display());
        if util::file::create_dir(&path) == false {
            error!("Failed to create directory {}", path.display());
            return Err("File IO Error".into());
        }
    }

    /* Create Template Sub Directories */
    for dir in vec![constants::configuration::DOCKER_TEMPLATES_CONFIG_FOLDER] {
        let path = PathBuf::from(&home)
            .join(constants::configuration::TEMPLATES_CONFIG_FOLDER)
            .join(dir);
        debug!("Creating project folder {}", path.display());
        if util::file::create_dir(&path) == false {
            error!("Failed to create directory {}", path.display());
            return Err("File IO Error".into());
        }
    }

    Ok(())
}

fn write_default_files(
    config: &ProjectConfiguration,
    home: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    /* Write Default Files */
    for (path, content) in vec![(template::ENV_SH_FILE_NAME, template::ENV_SH)] {
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
