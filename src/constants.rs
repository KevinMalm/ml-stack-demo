pub mod configuration {
    /* Modules */
    use log::debug;

    /* Environment Variables */
    pub const PROJECT_HOME_FOLDER_ENV: &str = "STACK_HOME";
    /* Folders */
    pub const SRC_FOLDER: &str = "src";
    pub const PROJECT_HOME_MODEL_CONFIG_FOLDER: &str = "config";
    pub const MODELS_FOLDER: &str = "models";
    pub const TEMPLATES_CONFIG_FOLDER: &str = "templates";
    pub const TERRAFORM_FOLDER: &str = "terraform";
    pub const DOCKER_TEMPLATES_CONFIG_FOLDER: &str = "docker";

    /* Files */
    pub const PROJECT_CONFIGURATION_FILE: &str = "ml.stack.toml";
    pub const MODEL_REPO_CONFIGURATION_FILE: &str = "model.repo.toml";
    pub const CONFIG_FILENAME_EXT: &str = ".toml";
    pub const DOCKER_FILE: &str = "Dockerfile";

    pub fn env_home() -> String {
        match std::env::var(PROJECT_HOME_FOLDER_ENV) {
            Ok(x) => x,
            Err(e) => {
                debug!("{:?}", e);
                panic!("Missing environment variable {}", PROJECT_HOME_FOLDER_ENV)
            }
        }
    }
}

pub mod placeholders {
    pub const PROJECT_NAME: &str = "<<PROJECT-NAME>>";
}
