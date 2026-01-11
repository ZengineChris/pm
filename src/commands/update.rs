use colored::Colorize;

use crate::cli::{Cli, UpdateArgs};
use crate::config::Config;
use crate::config::paths::expand_path;
use crate::error::Result;
use crate::git::update_repository;

pub fn execute(args: &UpdateArgs, cli: &Cli) -> Result<()> {
    let config = Config::load_or_default()?;

    let mut projects = config.projects.iter().collect::<Vec<_>>();

    if let Some(ref hosting) = args.hosting {
        projects.retain(|p| &p.hosting == hosting);
    }

    if let Some(ref name) = args.name {
        projects.retain(|p| &p.name == name);
    }

    if projects.is_empty() {
        if !cli.quiet {
            println!("No projects found.");
        }
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for project in projects {
        let repo_path = expand_path(&project.get_repo_path())?;

        if !repo_path.exists() {
            if !cli.quiet {
                println!(
                    "{} {} - skipped (not cloned)",
                    "⊗".yellow(),
                    project.name.cyan()
                );
            }
            continue;
        }

        if args.dry_run {
            if !cli.quiet {
                println!(
                    "{} Would update {}",
                    "ℹ".blue(),
                    project.name.cyan()
                );
            }
            continue;
        }

        if !cli.quiet && cli.verbose {
            println!("{} Updating {}...", "→".blue(), project.name.cyan());
        }

        match update_repository(&repo_path) {
            Ok(_) => {
                if !cli.quiet {
                    println!(
                        "{} Updated {}",
                        "✓".green().bold(),
                        project.name.cyan()
                    );
                }
                success_count += 1;
            }
            Err(e) => {
                if !cli.quiet {
                    eprintln!(
                        "{} Failed to update {}: {}",
                        "✗".red().bold(),
                        project.name.cyan(),
                        e
                    );
                }
                error_count += 1;
            }
        }
    }

    if !cli.quiet && !args.dry_run {
        println!();
        println!(
            "Summary: {} succeeded, {} failed",
            success_count.to_string().green(),
            error_count.to_string().red()
        );
    }

    Ok(())
}
