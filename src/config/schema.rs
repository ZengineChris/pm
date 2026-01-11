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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree_branch: Option<String>,
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
            worktree_branch: None,
            hosting,
            local_path,
            created_at: now,
            last_updated: now,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Get the actual filesystem path to the git repository.
    /// For worktree projects, this returns the path to the branch subdirectory.
    /// For regular projects, this returns the local_path as-is.
    pub fn get_repo_path(&self) -> String {
        if self.is_worktree {
            if let Some(ref branch) = self.worktree_branch {
                // Append the branch name to local_path
                let path = std::path::Path::new(&self.local_path).join(branch);
                path.to_string_lossy().to_string()
            } else {
                // Fallback: scan for subdirectories with .git if branch name not stored
                // This handles legacy projects created before worktree_branch was added
                if let Ok(entries) = std::fs::read_dir(&self.local_path) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                let subdir = entry.path();
                                if subdir.join(".git").exists() {
                                    return subdir.to_string_lossy().to_string();
                                }
                            }
                        }
                    }
                }
                // If no subdirectory with .git found, return local_path
                self.local_path.clone()
            }
        } else {
            self.local_path.clone()
        }
    }
}
