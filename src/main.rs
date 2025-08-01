mod args;
mod constants;
mod git;
mod mlflow;
mod options;
mod template;
mod util;

use args::{StackAction, StackArgs};

fn main() {
    /* Setup Logging */
    env_logger::init();
    /* Setup ML Flow */
    mlflow::init();
    /* Determine Action from CLI Args */
    if let Err(e) = match StackArgs::action() {
        StackAction::Deploy(x) => options::deploy::main(x),
        StackAction::Sync => options::sync::run(),
        StackAction::RunExperiment(x, y) => options::experiment::run(x, y),
        StackAction::InitializeProject(x) => options::init::project::main(&x),
        StackAction::InitializeModel(x) => options::init::model::main(&x),
        StackAction::InitializeModelRepository(x) => options::init::model_repository::main(&x),
        StackAction::InitializeModelInRepository(x) => options::init::model_in_repository::main(&x),
    } {
        panic!("{:?}", e);
    }
}
