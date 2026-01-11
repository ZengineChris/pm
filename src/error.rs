use thiserror::Error;

#[derive(Error, Debug)]
pub enum PmError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Project error: {0}")]
    Project(#[from] ProjectError),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Dialog error: {0}")]
    Dialog(#[from] dialoguer::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(String),

    #[error("Invalid config file: {0}")]
    Invalid(String),

    #[error("Failed to create config directory: {0}")]
    DirectoryCreation(String),

    #[error("Failed to parse config path: {0}")]
    PathParsing(String),
}

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Project '{0}' not found")]
    NotFound(String),

    #[error("Project '{0}' already exists")]
    AlreadyExists(String),

    #[error("Invalid project name: {0}")]
    InvalidName(String),

    #[error("Invalid repository URL: {0}")]
    InvalidUrl(String),

    #[error("Hosting '{0}' not configured")]
    HostingNotFound(String),

    #[error("Failed to compute project path: {0}")]
    PathComputation(String),
}

pub type Result<T> = std::result::Result<T, PmError>;
