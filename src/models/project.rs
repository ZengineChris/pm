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
        let path_segments = parse_git_url(url)?;

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

/// Parse both HTTPS and SSH-style git URLs
/// Supports:
/// - HTTPS: https://github.com/user/repo.git
/// - SSH: git@github.com:user/repo.git
/// - SCP: user@host:path/to/repo.git
fn parse_git_url(url: &str) -> Result<Vec<String>> {
    // Try to detect SSH-style URL (git@host:path or user@host:path)
    if url.contains('@') && url.contains(':') && !url.starts_with("http") {
        // SSH format: git@github.com:user/repo.git or user@host:path/to/repo.git
        let parts: Vec<&str> = url.split(':').collect();
        if parts.len() < 2 {
            return Err(ProjectError::InvalidUrl(
                format!("Invalid SSH URL format: {}", url)
            ).into());
        }

        // Extract path after the colon
        let path = parts[1..].join(":");

        // Split path by '/' to get segments
        let segments: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Ok(segments)
    } else {
        // Try to parse as standard HTTPS URL
        let parsed = Url::parse(url).map_err(|e| {
            ProjectError::InvalidUrl(format!("Failed to parse URL '{}': {}", url, e))
        })?;

        let segments: Vec<String> = parsed
            .path_segments()
            .ok_or_else(|| ProjectError::InvalidUrl("URL has no path segments".to_string()))?
            .map(|s| s.to_string())
            .collect();

        Ok(segments)
    }
}
