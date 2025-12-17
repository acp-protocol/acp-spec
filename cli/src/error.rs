//! Error types for ACP

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, AcpError>;

/// ACP error types
#[derive(Error, Debug)]
pub enum AcpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Parse error: {message}")]
    Parse { message: String, file: Option<String>, line: Option<usize> },

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Variable not found: {0}")]
    VarNotFound(String),

    #[error("Cycle detected in variable inheritance: {0}")]
    CycleDetected(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("{0}")]
    Other(String),
}

impl AcpError {
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
            file: None,
            line: None,
        }
    }

    pub fn parse_at(message: impl Into<String>, file: impl Into<String>, line: usize) -> Self {
        Self::Parse {
            message: message.into(),
            file: Some(file.into()),
            line: Some(line),
        }
    }
}
