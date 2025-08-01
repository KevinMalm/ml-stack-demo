/* Modules */
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};
/* Imports */
use crate::{
    constants,
    mlflow::config::{model::ModelConfiguration, tag::Tag},
    util,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerFile {
    pub default_training: Option<String>,
    pub default_evaluation: Option<String>,
    pub default_predict: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Git {
    pub default_repository_url: String,
}

/// Model Repository Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfiguration {
    pub label: String,
    pub prefix: String,
    pub tag: Tag,
    pub description: String,
    pub git: Git,
    pub dockerfile: DockerFile,
}

impl ProjectConfiguration {
    /// Create a new Empty ProjectConfiguration
    /// Certain fields are populated with default values
    pub fn new(label: &str) -> ProjectConfiguration {
        ProjectConfiguration {
            label: util::formatting::format_folder(label).to_string(),
            prefix: "demo".to_string(),
            tag: Tag::new("project", &util::formatting::format_folder(label)),
            description: "Placeholder description".to_string(),
            git: Git {
                default_repository_url: "<Default Github Model Repository URL>".to_string(),
            },
            dockerfile: DockerFile {
                default_evaluation: Some(format!(
                    "{}",
                    PathBuf::from(constants::configuration::TEMPLATES_CONFIG_FOLDER)
                        .join(constants::configuration::DOCKER_TEMPLATES_CONFIG_FOLDER)
                        .join("eval.docker")
                        .display()
                )),
                default_predict: Some(format!(
                    "{}",
                    PathBuf::from(constants::configuration::TEMPLATES_CONFIG_FOLDER)
                        .join(constants::configuration::DOCKER_TEMPLATES_CONFIG_FOLDER)
                        .join("eval.predict")
                        .display()
                )),
                default_training: Some(format!(
                    "{}",
                    PathBuf::from(constants::configuration::TEMPLATES_CONFIG_FOLDER)
                        .join(constants::configuration::DOCKER_TEMPLATES_CONFIG_FOLDER)
                        .join("eval.train")
                        .display()
                )),
            },
        }
    }

    pub fn list_models(&self) -> Result<Vec<ModelConfiguration>, String> {
        let home = PathBuf::from(constants::configuration::env_home())
            .join(constants::configuration::PROJECT_HOME_MODEL_CONFIG_FOLDER);
        let mut models = Vec::with_capacity(124);
        /* Walk through Config */
        for src in match std::fs::read_dir(&home) {
            Ok(x) => x,
            Err(e) => {
                error!("IO Error while walking through {}", home.display());
                debug!("{:?}", e);
                return Err("IO Error while reading configuration folder".into());
            }
        } {
            /* Validate File */
            let entry = match src {
                Ok(x) => x.path(),
                Err(e) => {
                    error!("IO Error while walking through {}", home.display());
                    debug!("{:?}", e);
                    return Err("IO Error while reading configuration folder".into());
                }
            };
            if entry.is_file() == false {
                println!("Skipping {}", entry.display());
                continue;
            }
            /* Extract File Name */
            let filename = match entry.file_name() {
                Some(x) => x.to_string_lossy(),
                None => {
                    error!("IO Error while pulling file name for {}", entry.display());
                    return Err("IO Error while reading file".into());
                }
            };
            /* Validate File Name */
            if filename.ends_with(constants::configuration::CONFIG_FILENAME_EXT) == false {
                warn!("Unexpected file found at {}", entry.display());
                continue;
            }
            /* Load Model Config */
            models.push(ModelConfiguration::load_from(&entry)?);
        }
        Ok(models)
    }

    /// Load the Project Configuration file
    /// from the expected path determined from the Environment Variables
    pub fn load() -> Result<ProjectConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::env_home())
            .join(constants::configuration::PROJECT_CONFIGURATION_FILE);

        if path.exists() == false {
            error!("Missing Project Configuration at {}", path.display());
            return Err("Missing Project Configuration".into());
        }

        /* Load Configuration */
        let content = match std::fs::read_to_string(&path) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to read the contents of {}", path.display());
                debug!("{:?}", e);
                return Err("IO Error encountered while reading Project Configuration".into());
            }
        };

        /* Deserialize Content */
        match toml::from_str(&content) {
            Ok(x) => Ok(x),
            Err(e) => {
                error!("Invalid Project Configuration found at {}", path.display());
                debug!("{:?}", e);
                return Err("Invalid Project Configuration".into());
            }
        }
    }

    /// Save the Project Configuration to the default location
    pub fn save(self) -> Result<ProjectConfiguration, String> {
        /* Determine Path */
        let path = PathBuf::from(constants::configuration::env_home())
            .join(constants::configuration::PROJECT_CONFIGURATION_FILE);
        /* Save */
        self.save_to(path)
    }

    /// Save the Project Configuration to a specified path
    pub fn save_to(self, path: PathBuf) -> Result<ProjectConfiguration, String> {
        debug!("Saving Project Configuration into {}", path.display());
        /* Serialize Configuration */
        let serialized = match toml::to_string(&self) {
            Ok(x) => x,
            Err(e) => {
                error!("Invalid contents for TOML serialization");
                debug!("{:?}", e);
                return Err("Project Configuration failed to serialize".into());
            }
        };
        /* Save to file */
        let mut f = match File::create(&path) {
            Ok(x) => x,
            Err(e) => {
                error!(
                    "Error while creating project configuration file {}",
                    path.display()
                );
                debug!("{:?}", e);
                return Err("IO error while creating Project Configuration file".into());
            }
        };

        if let Err(e) = f.write_all(serialized.as_bytes()) {
            error!(
                "Error while writing project configuration file {}",
                path.display()
            );
            debug!("{:?}", e);
            return Err("IO error while writing Project Configuration file".into());
        }
        Ok(self)
    }
}
