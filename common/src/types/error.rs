//! The error type that is shared between the client and the server

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// TODO: have an enum with multiple variants

/// Generic error type with a message
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("{error}")]
pub struct ApiError {
    /// The error message
    pub error: Cow<'static, str>,
}

impl ApiError {
    /// Get the error message
    pub fn message(&self) -> &str {
        &self.error
    }
}
