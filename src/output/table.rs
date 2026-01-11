use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::config::Project;
use crate::error::Result;
use crate::output::Formatter;

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "DESCRIPTION")]
    description: String,
    #[tabled(rename = "HOSTING")]
    hosting: String,
    #[tabled(rename = "WORKTREE")]
    worktree: String,
    #[tabled(rename = "PATH")]
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

        let table = Table::new(rows)
            .with(Style::empty())
            .to_string();
        Ok(table)
    }
}
