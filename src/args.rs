use clap::Parser;

pub enum StackAction {
    Sync,
    Deploy(Option<String>),
    InitializeModelRepository(String),
    InitializeModelInRepository(String),
    InitializeModel(String),
    InitializeProject(String),
    RunExperiment(String, bool),
}

/// ML Flow Stack builder CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct StackArgs {
    /// Deploy the active project's model
    #[arg(short, long)]
    deploy: Option<Option<String>>,

    /// Initialize a component for the project
    #[arg(short, long)]
    init: Option<Option<String>>,

    /// Flag to initialize a model
    #[arg(long)]
    model: Option<String>,

    /// Flag to initialize a model repository
    #[arg(long)]
    repo: Option<Option<String>>,

    /// Run an experiment
    #[arg(short, long)]
    run: Option<String>,

    /// Sync the models to ML Flow
    #[arg(short, long, action)]
    sync: bool,

    /// Build the docker image with no caching
    #[arg(short, long, action)]
    no_cache: bool,
}

impl StackArgs {
    pub fn action() -> StackAction {
        let args = StackArgs::parse();
        args.validate();
        /* Check for Init Group */
        if let Some(x) = &args.init {
            let secondary_init = args.repo.is_some() || args.model.is_some();
            /* Init a Project */
            if let Some(x) = x {
                if secondary_init {
                    panic!("--init ... can not be used with --repo or --model in tandem. See --help for details");
                }
                return StackAction::InitializeProject(x.clone());
            }
            if let Some(Some(_)) = &args.repo {
                if args.model.is_some() {
                    panic!(
                    "--repo ... and --model ... can not be used in tandem. See --help for details"
                );
                }
            }
            /* Init a Model Repo */
            if let Some(x) = &args.repo {
                if let Some(x) = x {
                    return StackAction::InitializeModelRepository(x.clone());
                }
            }
            /* Init a new Model */
            if let Some(x) = &args.model {
                /* Init a new Model in the Model Repository */
                if let Some(None) = &args.repo {
                    return StackAction::InitializeModelInRepository(x.clone());
                }
                /* Init a new Model in the Deployment Repository */
                return StackAction::InitializeModel(x.clone());
            }
            panic!("--init was called with an unexpected combination. See --help fro details")
        }
        /* Check for Deploy Group */
        if let Some(x) = args.deploy {
            return StackAction::Deploy(x.clone());
        }
        /* Check for Run Experiment Group */
        if let Some(x) = &args.run {
            return StackAction::RunExperiment(x.clone(), args.no_cache);
        }
        /* Check for syncing to ML Flow */
        if args.sync {
            return StackAction::Sync;
        }
        panic!("No action provided. See --help")
    }

    fn validate(&self) {
        let mut i: usize = 0;
        for x in vec![
            self.deploy.is_some(),
            self.run.is_some(),
            self.sync,
            self.init.is_some(),
        ] {
            if x {
                i += 1;
            }
        }
        match i {
            0 => {
                println!("No action provided. See --help")
            }
            1 => {}
            _ => {
                println!("Multiple actions provided. See --help")
            }
        }
    }
}
