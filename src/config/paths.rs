use std::path::PathBuf;

use crate::error::{ConfigError, Result};

pub fn get_config_dir() -> Result<PathBuf> {
    let base_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config)
    } else {
        dirs::home_dir()
            .ok_or_else(|| ConfigError::PathParsing("Could not determine home directory".to_string()))?
            .join(".config")
    };
    Ok(base_dir.join("pm"))
}

pub fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("projects.toml"))
}

pub fn expand_path(path: &str) -> Result<PathBuf> {
    let expanded = shellexpand::tilde(path);
    Ok(PathBuf::from(expanded.as_ref()))
}
