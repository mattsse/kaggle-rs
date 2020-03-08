use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaggleError {
    #[error("File not found {0}")]
    FileNotFound(PathBuf),
    #[error("Metadata error: {}", msg)]
    Metadata { msg: String },
}
