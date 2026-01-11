use std::path::PathBuf;

use crate::error::{ConfigError, Result};

pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| ConfigError::PathParsing("Could not determine config directory".to_string()))?
        .join("pm");
    Ok(config_dir)
}

pub fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("projects.toml"))
}

pub fn expand_path(path: &str) -> Result<PathBuf> {
    let expanded = shellexpand::tilde(path);
    Ok(PathBuf::from(expanded.as_ref()))
}
