use std::fs;

use crate::config::paths::{get_config_dir, get_config_file_path};
use crate::config::schema::Config;
use crate::error::{ConfigError, Result};

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_file_path()?;

        if !config_path.exists() {
            return Err(ConfigError::NotFound(
                config_path.to_string_lossy().to_string(),
            )
            .into());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default() -> Result<Self> {
        match Self::load() {
            Ok(config) => Ok(config),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = get_config_dir()?;

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| {
                ConfigError::DirectoryCreation(format!("{}: {}", config_dir.display(), e))
            })?;
        }

        let config_path = get_config_file_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn init(force: bool) -> Result<()> {
        let config_path = get_config_file_path()?;

        if config_path.exists() && !force {
            return Err(ConfigError::Invalid(
                "Config file already exists. Use --force to overwrite.".to_string(),
            )
            .into());
        }

        let config = Config::default();
        config.save()?;
        Ok(())
    }
}
