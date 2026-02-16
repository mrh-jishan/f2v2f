use thiserror::Error;
use std::io;
use std::fmt;

#[derive(Error, Debug)]
pub enum F2V2FError {
    #[error("IO Error: {0}")]
    Io(String),

    #[error("Image error: {0}")]
    ImageError(String),

    #[error("Video error: {0}")]
    VideoError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Data integrity error: {0} (expected: {1}, got: {2})")]
    IntegrityError(String, String, String),

    #[error("Interrupted: {0}")]
    Interrupted(String),

    #[error("Out of memory: {0}")]
    OutOfMemory(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<io::Error> for F2V2FError {
    fn from(err: io::Error) -> Self {
        F2V2FError::Io(err.to_string())
    }
}

impl From<image::ImageError> for F2V2FError {
    fn from(err: image::ImageError) -> Self {
        F2V2FError::ImageError(err.to_string())
    }
}

impl From<std::fmt::Error> for F2V2FError {
    fn from(err: std::fmt::Error) -> Self {
        F2V2FError::ConfigError(format!("Formatting error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, F2V2FError>;

/// Error context for tracking operation progress
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file_offset: u64,
    pub frame_number: u64,
    pub operation: String,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operation: {}, Frame: {}, File Offset: {} bytes",
            self.operation, self.frame_number, self.file_offset
        )
    }
}
