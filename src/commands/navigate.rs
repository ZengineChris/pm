use crate::cli::NavigateArgs;
use crate::config::Config;
use crate::config::paths::expand_path;
use crate::error::{ProjectError, Result};

pub fn execute(args: NavigateArgs) -> Result<()> {
    let config = Config::load_or_default()?;

    let project = config
        .find_project(&args.name)
        .ok_or_else(|| ProjectError::NotFound(args.name.clone()))?;

    let expanded_path = expand_path(&project.get_repo_path())?;

    println!("cd {}", expanded_path.display());

    Ok(())
}
