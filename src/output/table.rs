use tabled::{Table, Tabled};

use crate::config::Project;
use crate::error::Result;
use crate::output::Formatter;

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Hosting")]
    hosting: String,
    #[tabled(rename = "Worktree")]
    worktree: String,
    #[tabled(rename = "Path")]
    path: String,
}

pub struct TableFormatter;

impl Formatter for TableFormatter {
    fn format(&self, projects: &[Project]) -> Result<String> {
        if projects.is_empty() {
            return Ok("No projects found.".to_string());
        }

        let rows: Vec<ProjectRow> = projects
            .iter()
            .map(|p| ProjectRow {
                name: p.name.clone(),
                description: p.description.clone().unwrap_or_else(|| "-".to_string()),
                hosting: p.hosting.clone(),
                worktree: if p.is_worktree { "yes" } else { "no" }.to_string(),
                path: p.local_path.clone(),
            })
            .collect();

        let table = Table::new(rows).to_string();
        Ok(table)
    }
}
