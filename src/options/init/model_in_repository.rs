use std::path::PathBuf;

use crate::{constants, git, template, util};
use log::{debug, error, info};

pub fn main(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    /* Checkout the new Branch */
    let branch_name = format!("{}@0.0.0", util::formatting::format_folder(model));
    println!("{}", branch_name);

    /* Create Branch */
    if git::checkout_orphan(&branch_name)? == false {
        error!("Failed to checkout a new orphan branch in the local repository");
        return Err("GIT checkout error".into());
    }

    /* Validate the user isn't going to loose work */
    if git::check_diff()? {
        error!("Uncommitted changes were detected in the repo. Can not continue");
        return Err("GIT uncommitted changes".into());
    }

    /* Delete any files from the prior branch we don't want */
    flush_content()?;

    /* Create the Empty Project */
    write_sample()?;

    /* Commit the empty branch */
    if git::commit(&format!("Initial commit of {}", branch_name))? == false {
        error!("Failed to add a new commit to the orphan branch in the local repository");
        return Err("GIT commit error".into());
    }

    /* Push the local branch to the Remote repo */
    if git::publish_branch(&branch_name)? == false {
        error!("Failed to publish the new orphan branch in the local repository");
        return Err("GIT publish error".into());
    }

    info!(
        "Successfully established a new model {} under {}",
        model, branch_name
    );
    Ok(())
}

fn write_sample() -> Result<(), Box<dyn std::error::Error>> {
    /* Create Directories */
    for dir in vec![constants::configuration::SRC_FOLDER] {
        let path = PathBuf::from(dir);
        if util::file::create_dir(&path) == false {
            error!("Failed to create directory {}", path.display());
            return Err("File IO Error".into());
        }
    }
    /* Write Home files */
    for (name, content) in vec![("requirements.txt", template::REPO_MODEL_REQUIREMENTS_TXT)] {
        let path = PathBuf::from(name);

        if util::file::write_content(&path, &content) == false {
            error!("Failed to write into {}", path.display());
            return Err("File IO Error".into());
        }
    }
    /* Write SRC files */
    for (name, content) in vec![("__main__.py", template::REPO_MODEL_MAIN_PY)] {
        let path = PathBuf::from(constants::configuration::SRC_FOLDER).join(name);

        if util::file::write_content(&path, &content) == false {
            error!("Failed to write into {}", path.display());
            return Err("File IO Error".into());
        }
    }
    Ok(())
}

fn flush_content() -> Result<(), Box<dyn std::error::Error>> {
    let allowed_files = vec![".gitignore", "README.md"];
    let allowed_folders = vec![".git"];

    let dir = PathBuf::from(".");
    /* Walk through all templates */
    for src in match std::fs::read_dir(&dir) {
        Ok(x) => x,
        Err(e) => {
            error!("IO Error while walking through {}", dir.display());
            debug!("{:?}", e);
            return Err("IO Error while reading configuration folder".into());
        }
    } {
        /* Validate File */
        let entry = match src {
            Ok(x) => x.path(),
            Err(e) => {
                error!("IO Error while walking through {}", dir.display());
                debug!("{:?}", e);
                return Err("IO Error while reading configuration folder".into());
            }
        };
        /* Filter for allowed files */
        let allowed_items = match entry.is_file() {
            true => &allowed_files,
            false => &allowed_folders,
        };
        /* Extract File Name */
        let filename = match entry.file_name() {
            Some(x) => format!("{}", x.to_string_lossy()),
            None => {
                error!("IO Error while pulling file name for {}", entry.display());
                return Err("IO Error while reading file".into());
            }
        };

        /* Check if we can skip */
        let mut skip = false;
        for x in allowed_items {
            if x == &filename {
                skip = true;
                break;
            }
        }

        if skip {
            debug!("Skipping {}", filename);
            continue;
        }

        /* Delete... */
        if util::file::remove(&entry) == false {
            return Err("IO error while deleting".into());
        }
    }
    Ok(())
}
