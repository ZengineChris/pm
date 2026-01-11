use colored::Colorize;

use crate::cli::{Cli, CreateArgs};
use crate::config::{Config, Project};
use crate::config::paths::expand_path;
use crate::error::Result;
use crate::git::clone_repository;
use crate::models::compute_project_path;

pub fn execute(args: &CreateArgs, cli: &Cli) -> Result<()> {
    let mut config = Config::load_or_default()?;

    let hosting = if let Some(ref h) = args.hosting {
        h.clone()
    } else if let Some(ref url) = args.repo {
        config.infer_hosting_from_url(url)
    } else {
        "custom".to_string()
    };

    let local_path = compute_project_path(
        &config,
        &hosting,
        args.repo.as_deref(),
        args.path.as_deref(),
        args.worktree,
    )?;

    let mut worktree_branch = None;

    if args.clone {
        if let Some(ref repo_url) = args.repo {
            if !cli.quiet {
                println!("{} Cloning repository...", "→".blue());
            }

            let expanded_path = expand_path(&local_path)?;

            worktree_branch = clone_repository(repo_url, &expanded_path, args.worktree)?;

            if !cli.quiet {
                println!("{} Repository cloned", "✓".green().bold());
            }
        } else if !cli.quiet {
            println!(
                "{} Cannot clone without repository URL",
                "!".yellow()
            );
        }
    }

    let mut project = Project::new(args.name.clone(), hosting.clone(), local_path.clone());
    project.description = args.description.clone();
    project.repository_url = args.repo.clone();
    project.is_worktree = args.worktree;
    project.worktree_branch = worktree_branch;

    config.add_project(project)?;
    config.save()?;

    if !cli.quiet {
        println!(
            "{} Created project '{}'",
            "✓".green().bold(),
            args.name.cyan()
        );
        println!("  Hosting: {}", hosting.bright_black());
        println!("  Path: {}", local_path.bright_black());
    }

    Ok(())
}
