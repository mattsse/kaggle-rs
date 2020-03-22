use crate::models::Error;
use std::fmt;
use std::path::PathBuf;
use thiserror::Error;

/// Describes API errors
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Other(u16),
    ServerError(Error),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized request to API"),
            ApiError::Other(s) => write!(f, "Kaggle API reported error code {}", s),
            ApiError::ServerError(err) => err.fmt(f),
        }
    }
}

#[derive(Error, Debug)]
pub enum KaggleError {
    #[error("File not found {0}")]
    FileNotFound(PathBuf),
    #[error("Metadata error: {}", msg)]
    Metadata { msg: String },
    #[error(transparent)]
    Api {
        #[from]
        err: ApiError,
    },
}

impl KaggleError {
    pub(crate) fn meta(msg: impl ToString) -> Self {
        KaggleError::Metadata {
            msg: msg.to_string(),
        }
    }
}
