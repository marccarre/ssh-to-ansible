use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to parse SSH configuration: {0}")]
    Parsing(#[from] ssh2_config::SshParserError),

    #[error("Failed to serialise to YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Failed file operation: {0}")]
    Io(#[from] std::io::Error),
}
