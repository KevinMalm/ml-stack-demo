use log::error;

use crate::mlflow;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let models = match mlflow::model_registry::list() {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to retrieve a list of the active models");
            return Err(e.into());
        }
    };

    for x in &models {
        println!(
            "{} / {} / {}",
            x.name, x.creation_timestamp, x.last_updated_timestamp
        );
    }
    Ok(())
}
