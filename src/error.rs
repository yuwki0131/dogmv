use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Custom error type for dogmv application
#[derive(Debug, Error)]
pub enum DogmvError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid UTF-8 in file: {0}")]
    InvalidUtf8(PathBuf),

    #[error("Not a markdown file: {0}")]
    #[allow(dead_code)]
    NotMarkdownFile(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Directory read error: {0}")]
    #[allow(dead_code)]
    DirectoryReadError(String),

    #[error("Rendering error: {0}")]
    #[allow(dead_code)]
    RenderingError(String),

    #[error("Invalid path: {0}")]
    #[allow(dead_code)]
    InvalidPath(String),
}

/// Convenient Result type alias for dogmv operations
pub type Result<T> = std::result::Result<T, DogmvError>;
