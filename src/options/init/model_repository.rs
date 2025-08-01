/* Imports */
use log::info;

/* Local Imports */
use crate::git;

pub fn main(project: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let config = ModelRepositoryConfiguration::new(project, vec![Tag::new("Project", project)]);

    /* Ensure we are checked out to main branch */
    git::checkout(&git::head_branch_name()?)?;

    //config.save()?;
    info!(
        "Successfully initialized project repository for {}",
        project
    );
    return Ok(());
}
