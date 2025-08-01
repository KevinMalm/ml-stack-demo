use log::{debug, error};
use std::process::{Command, Output};

pub fn head_branch_name() -> Result<String, Box<dyn std::error::Error>> {
    /* git remote show origin | sed -n '/HEAD branch/s/.*: //p' */
    let args: Vec<&str> = "remote show origin".split(' ').collect();
    let output = git_cmd(Command::new("git").args(args), "Failed to find git origin")?;

    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    for x in result.split("\n").map(|x| x.trim()) {
        if x.starts_with("HEAD branch:") {
            return Ok(x[(x.find(":").unwrap() + 1)..].trim().to_string());
        }
    }
    Err("Failed to find HEAD branch in repository".into())
}

pub fn branch_name() -> Result<String, Box<dyn std::error::Error>> {
    /* git branch --show-current */

    let output = git_cmd(
        Command::new("git").args(vec!["branch", "--show-current"]),
        "Failed to find the active git branch",
    )?;

    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

    Ok(result.trim().to_string())
}

pub fn long_hash() -> Result<String, Box<dyn std::error::Error>> {
    /* git rev-parse HEAD */

    let output = git_cmd(
        Command::new("git").args(vec!["rev-parse", "HEAD"]),
        "Failed to find the active git branch",
    )?;

    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

    Ok(result.trim().to_string())
}

pub fn short_hash() -> Result<String, Box<dyn std::error::Error>> {
    /* git rev-parse --short HEAD  */

    let output = git_cmd(
        Command::new("git").args(vec!["rev-parse", "--short", "HEAD"]),
        "Failed to find the active git branch",
    )?;

    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

    Ok(result.trim().to_string())
}

fn git_cmd(cmd: &mut Command, error_message: &str) -> Result<Output, Box<dyn std::error::Error>> {
    match cmd.output() {
        Ok(x) => Ok(x),
        Err(e) => {
            debug!("{:?}", e);
            return Err(error_message.into());
        }
    }
}

fn handle_standard_result(output: &Output) -> Result<bool, Box<dyn std::error::Error>> {
    /* Print STDOUT */
    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    println!("--------");
    println!("{}", result);
    println!("--------");

    /* Check Error Code */
    if output.status.success() == false {
        debug!("{}", String::from_utf8_lossy(&output.stderr));
        error!(
            "Process Exited with code: {}",
            output.status.code().unwrap_or(-1)
        );
    }
    Ok(output.status.success())
}

pub fn checkout(brach: &str) -> Result<bool, Box<dyn std::error::Error>> {
    /* git checkout main / master */
    debug!("Now checking out {}", brach);
    let output = git_cmd(
        Command::new("git").args(vec!["checkout", brach]),
        "Failed to checkout git branch",
    )?;

    handle_standard_result(&output)
}

pub fn checkout_orphan(brach: &str) -> Result<bool, Box<dyn std::error::Error>> {
    /* git checkout --orphan <name> */
    debug!("Now checking out {} (orphan)", brach);
    let output = git_cmd(
        Command::new("git").args(vec!["checkout", "--orphan", brach]),
        "Failed to checkout git branch",
    )?;

    handle_standard_result(&output)
}

pub fn commit(message: &str) -> Result<bool, Box<dyn std::error::Error>> {
    /* git add .           */
    /* git commit -m "..." */

    debug!("Now committing to git with message '{}'", message);
    let output = git_cmd(
        Command::new("git").args(vec!["add", "."]),
        "Failed to add changes to the git commit",
    )?;

    let _ = handle_standard_result(&output)?;

    let output = git_cmd(
        Command::new("git").args(vec!["commit", "-m", &format!("{:?}", message)]),
        "Failed to push the git commit",
    )?;

    handle_standard_result(&output)
}

pub fn publish_branch(brach: &str) -> Result<bool, Box<dyn std::error::Error>> {
    /* git checkout --orphan <name> */
    debug!("Now publishing the branch {}", brach);
    let output = git_cmd(
        Command::new("git").args(vec!["push", "-u", "origin", brach]),
        "Failed to push git branch",
    )?;

    handle_standard_result(&output)
}

pub fn check_diff() -> Result<bool, Box<dyn std::error::Error>> {
    /* git diff */
    debug!("Now checking for uncommitted changes in the active branch");
    let output = git_cmd(
        Command::new("git").args(vec!["diff"]),
        "Failed to push git branch",
    )?;

    let result: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    println!("--------");
    println!("{}", result);
    println!("--------");

    return Ok(result.len() > 0);
}
