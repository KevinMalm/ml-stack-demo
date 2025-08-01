/* Modules */
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};
/* Imports */
use crate::{constants, mlflow::config::tag::Tag};

/// Model Repository Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRepositoryConfiguration {
    pub prefix: String,
    pub default_tags: Vec<Tag>,
}

impl ModelRepositoryConfiguration {
    pub fn new(prefix: &str, tag: Vec<Tag>) -> ModelRepositoryConfiguration {
        ModelRepositoryConfiguration {
            prefix: prefix.to_string(),
            default_tags: tag,
        }
    }

    pub fn load() -> Result<ModelRepositoryConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::MODEL_REPO_CONFIGURATION_FILE);
        if path.exists() == false {
            error!(
                "Missing Model Repository Configuration at {}",
                path.display()
            );
            return Err("Missing Model Repository Configuration".into());
        }

        /* Load Configuration */
        let content = match std::fs::read_to_string(&path) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to read the contents of {}", path.display());
                debug!("{:?}", e);
                return Err(
                    "IO Error encountered while reading Model Repository Configuration".into(),
                );
            }
        };

        /* Deserialize Content */
        match toml::from_str(&content) {
            Ok(x) => Ok(x),
            Err(e) => {
                error!(
                    "Invalid Model Repository Configuration found at {}",
                    path.display()
                );
                debug!("{:?}", e);
                return Err("Invalid Model Repository Configuration".into());
            }
        }
    }

    pub fn save(self) -> Result<ModelRepositoryConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::MODEL_REPO_CONFIGURATION_FILE);
        /* Serialize Configuration */
        let serialized = match toml::to_string(&self) {
            Ok(x) => x,
            Err(e) => {
                error!("Invalid contents for TOML serialization");
                debug!("{:?}", e);
                return Err("Model Repository Configuration failed to serialize".into());
            }
        };
        /* Save to file */
        let mut f = match File::create(&path) {
            Ok(x) => x,
            Err(e) => {
                error!(
                    "Error while creating model configuration file {}",
                    path.display()
                );
                debug!("{:?}", e);
                return Err("IO error while creating Model Repository Configuration file".into());
            }
        };

        if let Err(e) = f.write_all(serialized.as_bytes()) {
            error!(
                "Error while writing model configuration file {}",
                path.display()
            );
            debug!("{:?}", e);
            return Err("IO error while writing Model Repository Configuration file".into());
        }
        Ok(self)
    }
}
