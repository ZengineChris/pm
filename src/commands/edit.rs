use colored::Colorize;

use crate::cli::{Cli, EditArgs};
use crate::config::Config;
use crate::error::{ProjectError, Result};

pub fn execute(args: &EditArgs, cli: &Cli) -> Result<()> {
    let mut config = Config::load_or_default()?;

    let project = config
        .find_project_mut(&args.name)
        .ok_or_else(|| ProjectError::NotFound(args.name.clone()))?;

    let mut modified = false;

    if let Some(ref desc) = args.description {
        project.description = Some(desc.clone());
        modified = true;
    }

    if let Some(ref repo) = args.repo {
        project.repository_url = Some(repo.clone());
        modified = true;
    }

    if args.worktree {
        project.is_worktree = true;
        modified = true;
    }

    if args.no_worktree {
        project.is_worktree = false;
        modified = true;
    }

    if let Some(ref path) = args.path {
        project.local_path = path.clone();
        modified = true;
    }

    if let Some(ref new_name) = args.name_new {
        project.name = new_name.clone();
        modified = true;
    }

    if modified {
        project.update_timestamp();
        config.save()?;

        if !cli.quiet {
            println!(
                "{} Updated project '{}'",
                "✓".green().bold(),
                args.name.cyan()
            );
        }
    } else if !cli.quiet {
        println!("No changes made.");
    }

    Ok(())
}
