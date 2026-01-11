use crate::config::Config;
use crate::config::paths::expand_path;
use crate::error::{ProjectError, Result};
use url::Url;

pub fn compute_project_path(
    config: &Config,
    hosting: &str,
    repo_url: Option<&str>,
    custom_path: Option<&str>,
    _is_worktree: bool,
) -> Result<String> {
    if let Some(custom) = custom_path {
        return Ok(custom.to_string());
    }

    let hosting_config = config
        .hostings
        .get(hosting)
        .ok_or_else(|| ProjectError::HostingNotFound(hosting.to_string()))?;

    let base_path = expand_path(&hosting_config.base_path)?;

    if let Some(url) = repo_url {
        let parsed = Url::parse(url).map_err(|e| {
            ProjectError::InvalidUrl(format!("Failed to parse URL '{}': {}", url, e))
        })?;

        let path_segments: Vec<&str> = parsed
            .path_segments()
            .ok_or_else(|| ProjectError::InvalidUrl("URL has no path segments".to_string()))?
            .collect();

        if path_segments.is_empty() {
            return Err(
                ProjectError::InvalidUrl("URL path is empty".to_string()).into()
            );
        }

        let mut full_path = base_path;

        for segment in path_segments {
            let clean_segment = segment.trim_end_matches(".git");
            if !clean_segment.is_empty() {
                full_path = full_path.join(clean_segment);
            }
        }

        Ok(full_path.to_string_lossy().to_string())
    } else {
        Err(ProjectError::PathComputation(
            "Cannot compute path without repository URL or custom path".to_string(),
        )
        .into())
    }
}
