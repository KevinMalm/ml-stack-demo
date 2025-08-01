/* Modules */
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};
/* Imports */
use crate::{constants, mlflow::config::tag::Tag, util};

/// Configuration for the live (production) instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveModelPointer {
    pub branch: String,
    pub version: String,
}

/// Configuration for the staging (production) instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingModelPointer {
    pub branch: String,
    pub version: Option<String>,
}

/// Model Repository Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfiguration {
    pub label: String,
    pub description: String,
    pub tags: Vec<Tag>,
    pub override_repository_url: Option<String>,
    pub production: Option<LiveModelPointer>,
    pub staging: Option<StagingModelPointer>,
}

impl ModelConfiguration {
    pub fn new(label: &str) -> ModelConfiguration {
        ModelConfiguration {
            label: label.to_string(),
            description: "Placeholder description".to_string(),
            tags: vec![],
            override_repository_url: None,
            production: Some(LiveModelPointer {
                branch: "<github branch>".to_string(),
                version: "version-tag".to_string(),
            }),
            staging: Some(StagingModelPointer {
                branch: "<github branch>".to_string(),
                version: Some("version-tag".to_string()),
            }),
        }
    }

    /// Determine the model's config file name
    fn to_file_name(label: &str) -> String {
        format!(
            "{}{}",
            util::formatting::to_dot_case(label),
            constants::configuration::CONFIG_FILENAME_EXT
        )
    }

    pub fn to_folder_name(&self) -> String {
        util::formatting::format_py_folder(&self.label)
    }

    pub fn load(label: &str) -> Result<ModelConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::env_home())
            .join(constants::configuration::PROJECT_HOME_MODEL_CONFIG_FOLDER)
            .join(ModelConfiguration::to_file_name(label));

        if path.exists() == false {
            error!("Missing Model Configuration at {}", path.display());
            return Err("Missing Model Configuration".into());
        }
        ModelConfiguration::load_from(&path)
    }

    pub fn load_from(path: &PathBuf) -> Result<ModelConfiguration, String> {
        /* Load Configuration */
        let content = match std::fs::read_to_string(&path) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to read the contents of {}", path.display());
                debug!("{:?}", e);
                return Err("IO Error encountered while reading Model Configuration".into());
            }
        };

        /* Deserialize Content */
        match toml::from_str(&content) {
            Ok(x) => Ok(x),
            Err(e) => {
                error!("Invalid Model Configuration found at {}", path.display());
                debug!("{:?}", e);
                return Err("Invalid Model Configuration".into());
            }
        }
    }

    pub fn save(self) -> Result<ModelConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::env_home())
            .join(constants::configuration::PROJECT_HOME_MODEL_CONFIG_FOLDER)
            .join(ModelConfiguration::to_file_name(&self.label));
        /* Serialize Configuration */
        let serialized = match toml::to_string(&self) {
            Ok(x) => x,
            Err(e) => {
                error!("Invalid contents for TOML serialization");
                debug!("{:?}", e);
                return Err("Model Configuration failed to serialize".into());
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
                return Err("IO error while creating Model Configuration file".into());
            }
        };

        if let Err(e) = f.write_all(serialized.as_bytes()) {
            error!(
                "Error while writing model configuration file {}",
                path.display()
            );
            debug!("{:?}", e);
            return Err("IO error while writing Model Configuration file".into());
        }
        Ok(self)
    }
}
