use colored::Colorize;
use dialoguer::Confirm;

use crate::cli::{Cli, DeleteArgs};
use crate::config::Config;
use crate::config::paths::expand_path;
use crate::error::Result;

pub fn execute(args: &DeleteArgs, cli: &Cli) -> Result<()> {
    let mut config = Config::load_or_default()?;

    let project = config.find_project(&args.name).cloned();

    if project.is_none() {
        return Err(crate::error::ProjectError::NotFound(args.name.clone()).into());
    }

    let project = project.unwrap();

    let confirmed = if args.force {
        true
    } else {
        Confirm::new()
            .with_prompt(format!("Delete project '{}'?", args.name))
            .default(false)
            .interact()?
    };

    if !confirmed {
        if !cli.quiet {
            println!("Cancelled.");
        }
        return Ok(());
    }

    config.remove_project(&args.name)?;
    config.save()?;

    if !cli.quiet {
        println!(
            "{} Deleted project '{}'",
            "✓".green().bold(),
            args.name.cyan()
        );
    }

    if args.delete_files {
        let path = expand_path(&project.local_path)?;
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
            if !cli.quiet {
                println!(
                    "{} Deleted local files at {}",
                    "✓".green().bold(),
                    path.display().to_string().bright_black()
                );
            }
        } else if !cli.quiet {
            println!(
                "{} Local path does not exist: {}",
                "!".yellow(),
                path.display().to_string().bright_black()
            );
        }
    }

    Ok(())
}
