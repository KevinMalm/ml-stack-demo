use log::{debug, error};

use crate::{mlflow, options::sync};

pub fn main(model: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    /* Refresh model data */
    sync::run()?;

    let models = vec![model.unwrap()];

    Ok(())
}
