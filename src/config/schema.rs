use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{ProjectError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub settings: Settings,
    pub hostings: HashMap<String, Hosting>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hosting {
    pub base_path: String,
    pub url_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub is_worktree: bool,
    pub hosting: String,
    pub local_path: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl Default for Config {
    fn default() -> Self {
        let mut hostings = HashMap::new();

        hostings.insert(
            "github".to_string(),
            Hosting {
                base_path: "~/github.com".to_string(),
                url_pattern: "github.com".to_string(),
            },
        );

        hostings.insert(
            "gitlab".to_string(),
            Hosting {
                base_path: "~/gitlab.com".to_string(),
                url_pattern: "gitlab.com".to_string(),
            },
        );

        hostings.insert(
            "azure".to_string(),
            Hosting {
                base_path: "~/azure.com".to_string(),
                url_pattern: "azure.com".to_string(),
            },
        );

        hostings.insert(
            "custom".to_string(),
            Hosting {
                base_path: "~/git".to_string(),
                url_pattern: String::new(),
            },
        );

        Config {
            version: "1.0".to_string(),
            settings: Settings {
                default_output_format: OutputFormat::Table,
            },
            hostings,
            projects: Vec::new(),
        }
    }
}

impl Config {
    pub fn get_hosting_path(&self, hosting: &str) -> Option<&str> {
        self.hostings.get(hosting).map(|h| h.base_path.as_str())
    }

    pub fn infer_hosting_from_url(&self, url: &str) -> String {
        for (name, hosting) in &self.hostings {
            if !hosting.url_pattern.is_empty() && url.contains(&hosting.url_pattern) {
                return name.clone();
            }
        }
        "custom".to_string()
    }

    pub fn find_project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }

    pub fn find_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.name == name)
    }

    pub fn add_project(&mut self, project: Project) -> Result<()> {
        if self.find_project(&project.name).is_some() {
            return Err(ProjectError::AlreadyExists(project.name.clone()).into());
        }
        self.projects.push(project);
        Ok(())
    }

    pub fn remove_project(&mut self, name: &str) -> Result<Project> {
        let index = self
            .projects
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| ProjectError::NotFound(name.to_string()))?;
        Ok(self.projects.remove(index))
    }
}

impl Project {
    pub fn new(name: String, hosting: String, local_path: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            description: None,
            repository_url: None,
            is_worktree: false,
            hosting,
            local_path,
            created_at: now,
            last_updated: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }
}
