use std::io;
use std::path::PathBuf;

use thiserror::Error;

/// Errors produced by the Fily Core engine.
#[derive(Debug, Error)]
pub enum FilyError {
    /// The requested file or directory does not exist.
    #[error("path does not exist: {0}")]
    NotFound(PathBuf),

    /// The operation was not permitted.
    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// The requested destination already exists.
    #[error("destination already exists: {0}")]
    AlreadyExists(PathBuf),

    /// The supplied path is invalid.
    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),

    /// A filesystem operation failed.
    #[error("filesystem operation failed: {0}")]
    Io(#[from] io::Error),

    /// A generic operation failure.
    #[error("operation failed: {0}")]
    OperationFailed(String),
}

/// Convenient result type used throughout Fily Core.
pub type FilyResult<T> = Result<T, FilyError>;