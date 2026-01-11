use colored::Colorize;
use tabled::{Table, Tabled};

use crate::cli::{Cli, StatusArgs};
use crate::config::Config;
use crate::config::paths::expand_path;
use crate::error::Result;
use crate::git::get_repository_status;

#[derive(Tabled)]
struct StatusRow {
    #[tabled(rename = "Project")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Staged")]
    staged: String,
    #[tabled(rename = "Modified")]
    modified: String,
    #[tabled(rename = "Untracked")]
    untracked: String,
}

pub fn execute(args: &StatusArgs, cli: &Cli) -> Result<()> {
    let config = Config::load_or_default()?;

    let mut projects = config.projects.iter().collect::<Vec<_>>();

    if let Some(ref hosting) = args.hosting {
        projects.retain(|p| &p.hosting == hosting);
    }

    if let Some(ref name) = args.name {
        projects.retain(|p| &p.name == name);
    }

    let mut rows = Vec::new();

    for project in projects {
        let repo_path = expand_path(&project.get_repo_path())?;

        if !repo_path.exists() {
            rows.push(StatusRow {
                name: project.name.clone(),
                status: "Not cloned".yellow().to_string(),
                staged: "-".to_string(),
                modified: "-".to_string(),
                untracked: "-".to_string(),
            });
            continue;
        }

        match get_repository_status(&repo_path) {
            Ok(status) => {
                let status_str = if status.has_changes {
                    "Dirty".red().to_string()
                } else {
                    "Clean".green().to_string()
                };

                if args.dirty && !status.has_changes {
                    continue;
                }

                rows.push(StatusRow {
                    name: project.name.clone(),
                    status: status_str,
                    staged: status.staged.to_string(),
                    modified: status.modified.to_string(),
                    untracked: status.untracked.to_string(),
                });
            }
            Err(_) => {
                rows.push(StatusRow {
                    name: project.name.clone(),
                    status: "Not a git repo".yellow().to_string(),
                    staged: "-".to_string(),
                    modified: "-".to_string(),
                    untracked: "-".to_string(),
                });
            }
        }
    }

    if rows.is_empty() {
        if !cli.quiet {
            println!("No projects found.");
        }
        return Ok(());
    }

    let table = Table::new(rows).to_string();
    println!("{}", table);

    Ok(())
}
