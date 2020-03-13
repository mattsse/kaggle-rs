use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaggleError {
    #[error("File not found {0}")]
    FileNotFound(PathBuf),
    #[error("Metadata error: {}", msg)]
    Metadata { msg: String },
}

impl KaggleError {
    pub(crate) fn meta(msg: impl ToString) -> Self {
        KaggleError::Metadata {
            msg: msg.to_string(),
        }
    }
}
