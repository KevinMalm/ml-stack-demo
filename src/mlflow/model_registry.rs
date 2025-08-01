/* Package Imports */
use log::{debug, error};
use reqwest::blocking::Client;
use serde_json::json;

/* Local Imports */
use crate::mlflow::{
    config::tag::Tag,
    endpoint::{self, deserialize_response, read_response_text, MLFlowEndpoint},
};

#[derive(serde::Deserialize)]
pub struct ModelRegistryRecord {
    pub name: String,
    pub creation_timestamp: u64,
    pub last_updated_timestamp: u64,
}

#[derive(serde::Deserialize)]
pub struct ModelRegistryResponse {
    pub registered_models: Vec<ModelRegistryRecord>,
}

/// Create a new model in the Model Registry
pub fn create(
    model: &str,
    description: Option<String>,
    tags: Option<Vec<Tag>>,
) -> Result<(), String> {
    let client = Client::new();
    let endpoint = MLFlowEndpoint::RegisterModel.url();
    /* Make request */
    let body = json!({
        "name": model,
        "tags": tags,
        "description": description
    });
    let response = match client.post(&endpoint).json(&body).send() {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to make POST to {}", endpoint);
            debug!("{}", e);
            return Err("INVALID REQUEST".to_string());
        }
    };

    match response.status().is_success() {
        true => Ok(()),
        false => {
            debug!("{:?}", body);
            Err(endpoint::handle_error(response))
        }
    }
}

/// List all models in the Model Registry
pub fn list() -> Result<Vec<ModelRegistryRecord>, String> {
    let client = Client::new();
    let endpoint = MLFlowEndpoint::ListModels.url();
    /* Make request */
    let response = match client.get(&endpoint).send() {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to make POST to {}", endpoint);
            debug!("{}", e);
            return Err("INVALID REQUEST".to_string());
        }
    };

    match response.status().is_success() {
        true => {
            let msg = read_response_text(response)?;
            let arr: ModelRegistryResponse = deserialize_response(&msg)?;
            Ok(arr.registered_models)
        }
        false => Err(endpoint::handle_error(response)),
    }
}
