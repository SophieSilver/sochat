use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Generic error type with an message and a source
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("{error}")]
pub struct ApiError {
    pub error: String,
}
