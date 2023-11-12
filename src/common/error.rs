use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid user input for arg \"{arg}\". Reason: {reason}")]
    InvalidInput { arg: &'static str, reason: String },

    #[error("Failed to serialise to YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Failed I/O operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed string conversion: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
