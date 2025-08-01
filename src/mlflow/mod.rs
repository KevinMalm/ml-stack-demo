/* Modules */
pub mod config;
pub mod endpoint;
pub mod model_registry;
/* Imports */
use log::{debug, error};
use std::sync::OnceLock;

/// Environment Variable for the ML FLow Host Address
const HOST_NAME_ENV_VAR: &str = "ML_FLOW_HOST";
/// Environment Variable for the ML FLow Serving Port
const HOST_PORT_ENV_VAR: &str = "ML_FLOW_PORT";

/// Loaded Host name from the Environment Variables
pub static HOST: OnceLock<String> = OnceLock::new();
/// Loaded Host port from the Environment Variables
pub static PORT: OnceLock<usize> = OnceLock::new();
/// Host URL for any REST requests
pub static URL: OnceLock<String> = OnceLock::new();

/// Read an environment variable and returns the result.
/// Panics if the variable is not set
fn read_env_var(var: &str) -> Option<String> {
    match std::env::var(var) {
        Ok(x) => Some(x),
        Err(e) => {
            error!("Missing environment variable: {}", HOST_NAME_ENV_VAR);
            debug!("{:?}", e);
            return None;
        }
    }
}

/// Initialize the ML Flow static variables
pub fn init() {
    /* Read Environment Variables */
    let host = read_env_var(HOST_NAME_ENV_VAR).expect("Invalid environment configuration");
    let port = read_env_var(HOST_PORT_ENV_VAR).expect("Invalid environment configuration");
    /* Parse Environment Variables */
    let port = match port.parse::<usize>() {
        Ok(x) => x,
        Err(e) => {
            error!(
                "Invalid host port set in {}: {}",
                HOST_NAME_ENV_VAR, HOST_PORT_ENV_VAR
            );
            debug!("{:?}", e);
            panic!("Invalid environment configuration");
        }
    };
    /* Set Static Variables */
    if let Err(_) = HOST.set(host.clone()) {
        panic!("Failed to set the ML-Flow host variable");
    }
    if let Err(_) = PORT.set(port) {
        panic!("Failed to set the ML-Flow port variable");
    }
    if let Err(_) = URL.set(format!("http://{}:{}", host, port)) {
        panic!("Failed to set the ML-Flow port variable");
    }
}
