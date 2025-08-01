use std::{
    collections::HashSet,
    process::{Command, Stdio},
};

use log::{debug, error, info, warn};
use tempfile::TempDir;

use crate::{
    constants,
    mlflow::{
        self,
        config::{model::ModelConfiguration, project::ProjectConfiguration},
    },
    template, util,
};

pub fn run(branch: String, no_cache: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (model, version) = parse_model_tag(&branch)?;
    /* Load configuration */
    let project_config = ProjectConfiguration::load()?;
    let model_config = ModelConfiguration::load(&model)?;
    /* Debug */
    info!(
        "Deploying {}::{} under {}.{}",
        model, version, project_config.label, model_config.label
    );
    /* Create Temp Directory */
    let temp_dir = match tempfile::tempdir() {
        Ok(x) => x,
        Err(e) => {
            error!(
                "Failed to create the temporary directory for {}",
                model_config.label
            );
            debug!("{}", e);
            return Err("Failed to build docker image".into());
        }
    };

    /* Write Docker File */
    if let Err(e) = write_docker_file(&temp_dir, &model_config) {
        if let Err(e) = temp_dir.close() {
            error!("Failed to close the temporary directory");
            debug!("{}", e);
        }
        return Err(e);
    }

    /* Build Docker File */
    if let Err(e) = build_docker_file(&temp_dir, &branch, &model_config, &project_config, no_cache)
    {
        if let Err(e) = temp_dir.close() {
            error!("Failed to close the temporary directory");
            debug!("{}", e);
        }
        return Err(e);
    }
    Ok(())
}

fn parse_model_tag(branch: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let divider_index = match branch.find('@') {
        Some(i) => i,
        None => {
            error!("Invalid branch tag: {}", branch);
            return Err("Invalid Branch Name".into());
        }
    };

    Ok((
        branch[0..divider_index].to_string(),
        branch[divider_index + 1..].to_string(),
    ))
}

fn write_docker_file(
    temp_dir: &TempDir,
    model_config: &ModelConfiguration,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = temp_dir.path();
    debug!("Working Directory: {}", temp_path.display());

    if util::file::write_content(
        &temp_path.join(temp_path.join(constants::configuration::DOCKER_FILE)),
        template::RUN_EXPERIMENT_DOCKERFILE,
    ) == false
    {
        error!(
            "Failed to generate the docker build script for {}",
            model_config.label
        );
        return Err("Failed to write docker image".into());
    }
    Ok(())
}

fn build_docker_file(
    temp_dir: &TempDir,
    branch: &str,
    model_config: &ModelConfiguration,
    project_config: &ProjectConfiguration,
    no_cache: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = temp_dir.path();

    /* What are the Experiment Tags? */
    let tags = stringify_tags(model_config, project_config)?;
    /* Where is the source files at? */
    let repo = model_config
        .override_repository_url
        .clone()
        .unwrap_or(project_config.git.default_repository_url.clone());
    /* Whats the docker image tag? */
    let docker_tag = format!("{}-{}", &project_config.label, branch.replace("@", "-"));

    warn!("TODO: Fix the GIT-LIB hard coding");
    let mut cmd = Command::new("docker");
    let status = match (match no_cache {
        true => cmd.args(&[
            "build",
            "--no-cache",
            "--build-arg",
            &format!("MODEL_REPO={}", repo),
            "--build-arg",
            &format!("MODEL_NAME={}", branch),
            "--build-arg",
            &format!("MODEL_TAGS={}", tags),
            "--build-arg",
            &format!("GITHUB_LIB_BRANCH={}", "ml-stack-py"),
            "--build-arg",
            &format!("SERVER_URL={}", "host.docker.internal"),
            "--build-arg",
            &format!("GITHUB_LIB_REPO={}", repo),
            "--build-arg",
            &format!(
                "MLFLOW_HOST={}",
                mlflow::HOST
                    .get()
                    .expect("ML-Flow HOST was referenced before it was set")
            ),
            "--build-arg",
            &format!(
                "MLFLOW_PORT={}",
                mlflow::PORT
                    .get()
                    .expect("ML-Flow PORT was referenced before it was set")
            ),
            "-t",
            &docker_tag,
            temp_path.to_str().unwrap(),
        ]),
        false => cmd.args(&[
            "build",
            "--build-arg",
            &format!("MODEL_REPO={}", repo),
            "--build-arg",
            &format!("MODEL_NAME={}", branch),
            "--build-arg",
            &format!("MODEL_TAGS={}", tags),
            "--build-arg",
            &format!("GITHUB_LIB_BRANCH={}", "ml-stack-py"),
            "--build-arg",
            &format!("SERVER_URL={}", "host.docker.internal"),
            "--build-arg",
            &format!("GITHUB_LIB_REPO={}", repo),
            "-t",
            &docker_tag,
            temp_path.to_str().unwrap(),
        ]),
    })
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    {
        Ok(x) => x,
        Err(e) => {
            error!(
                "Failed to run the docker build command for {}",
                model_config.label
            );
            debug!("{}", e);
            return Err("Failed to build docker image".into());
        }
    };
    if status.success() == false {
        debug!("{:?}", status);
        return Err("Docker build exited not OK".into());
    }
    Ok(())
}

fn stringify_tags(
    model_config: &ModelConfiguration,
    project_config: &ProjectConfiguration,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut tags = HashSet::new();
    tags.insert(project_config.tag.clone());
    tags.extend(model_config.tags.clone());

    let json = match serde_json::to_string(&tags) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to serialize the experiment tags to JSON");
            debug!("{:?}", e);
            return Err("Experiment tags failed to serialize".into());
        }
    };

    Ok(json.replace('\\', "\\\\").replace('\"', "\\\""))
}
