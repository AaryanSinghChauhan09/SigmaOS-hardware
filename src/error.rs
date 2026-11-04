use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Service error: {0}")]
    Service(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for ComponentError {
    fn from(e: anyhow::Error) -> Self { ComponentError::Internal(e.to_string()) }
}

pub type Result<T> = std::result::Result<T, ComponentError>;
