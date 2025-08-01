/* Package Imports */
use crate::mlflow::URL;
use log::{debug, error, trace};
use serde::Deserialize;
use serde_json::Value;

/// ML Flow API version
const MODEL_VERSION: &str = "2.0";

pub enum MLFlowEndpoint {
    RegisterModel,
    ListModels,
    SearchModels,
}

impl MLFlowEndpoint {
    pub fn url(&self) -> String {
        let uri = match self {
            MLFlowEndpoint::RegisterModel => "registered-models/create",
            MLFlowEndpoint::ListModels => "registered-models/search",
            MLFlowEndpoint::SearchModels => {
                debug!("TODO: This should include search parameters");
                "registered-models/search"
            }
        };

        format!(
            "{}/api/{}/mlflow/{}",
            URL.get()
                .expect("HOST was referenced before being initialized"),
            MODEL_VERSION,
            uri
        )
    }
}

pub fn handle_error(response: reqwest::blocking::Response) -> String {
    match response.text() {
        Ok(s) => {
            trace!("Error message: {}", s);
            return match serde_json::from_str::<Value>(&s) {
                Ok(x) => match x.get("error_code") {
                    Some(x) => match x.as_str() {
                        Some(x) => x.to_string(),
                        None => {
                            debug!("Invalid String: {}", x);
                            return "No error code in message".to_string();
                        }
                    },
                    None => {
                        debug!("RAW response: {}", s);
                        return "No error code in message".to_string();
                    }
                },
                Err(e) => {
                    error!("{:?}", e);
                    return "Failed to decode body".to_string();
                }
            };
        }
        Err(e) => {
            error!("{:?}", e);
            return "Failed to decode body".to_string();
        }
    }
}

pub fn read_response_text(response: reqwest::blocking::Response) -> Result<String, String> {
    match response.text() {
        Ok(x) => Ok(x),
        Err(e) => {
            error!("{:?}", e);
            return Err("Failed to decode body".to_string());
        }
    }
}

pub fn deserialize_response<'a, T>(response: &'a str) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(&response) {
        Ok(x) => Ok(x),
        Err(e) => {
            error!("{:?}", e);
            return Err("Failed to decode body".to_string());
        }
    }
}
