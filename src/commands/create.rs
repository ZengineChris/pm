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

    let mut local_path = compute_project_path(
        &config,
        &hosting,
        args.repo.as_deref(),
        args.path.as_deref(),
        args.worktree,
    )?;

    if args.clone {
        if let Some(ref repo_url) = args.repo {
            if !cli.quiet {
                println!("{} Cloning repository...", "→".blue());
            }

            let expanded_path = expand_path(&local_path)?;

            if args.worktree {
                let default_branch = detect_default_branch_for_worktree(repo_url)?;
                let worktree_path = expanded_path
                    .join(&default_branch)
                    .to_string_lossy()
                    .to_string();
                local_path = worktree_path;
            }

            clone_repository(repo_url, &expanded_path, args.worktree)?;

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

fn detect_default_branch_for_worktree(url: &str) -> Result<String> {
    let mut remote = git2::Remote::create_detached(url)?;
    let connection = remote.connect_auth(git2::Direction::Fetch, None, None)?;

    let default_branch = connection
        .default_branch()?
        .as_str()
        .ok_or_else(|| git2::Error::from_str("Could not determine default branch"))?
        .to_string();

    let branch_name = default_branch
        .strip_prefix("refs/heads/")
        .unwrap_or(&default_branch)
        .to_string();

    Ok(branch_name)
}
