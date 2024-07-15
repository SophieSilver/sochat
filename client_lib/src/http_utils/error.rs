use std::{error::Error as StdError, fmt::Display, io};

use common::types::ApiError;
use reqwest::StatusCode;
use thiserror::Error;

// TODO: put this in common
/// Error when serializing or deserializing CBOR
#[derive(Debug, Error)]
#[error(transparent)]
pub enum CborError {
    Serialize(#[from] ciborium::ser::Error<io::Error>),
    Deserialize(#[from] ciborium::de::Error<io::Error>),
}

impl From<CborSerializeError> for CborError {
    fn from(value: CborSerializeError) -> Self {
        Self::Serialize(value.0)
    }
}

/// Error when serializing an object with CBOR
#[derive(Debug, Error)]
#[error(transparent)]
pub struct CborSerializeError(#[from] ciborium::ser::Error<io::Error>);

/// Error when trying to parse request body.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum ResponseError {
    /// Error that initiated from the request itself
    Request(#[from] reqwest::Error),

    /// Error while deserializing the response body with CBOR
    Deserialize(#[from] ciborium::de::Error<io::Error>),
}

#[derive(Debug)]
pub struct StatusError {
    pub status: StatusCode,
    pub source: Result<ApiError, ResponseError>,
}

impl Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "backend server returned an error code {}", self.status)
    }
}

impl std::error::Error for StatusError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        let source = match &self.source {
            Ok(api_error) => api_error as &dyn StdError,
            Err(response_error) => response_error as &dyn StdError,
        };

        Some(source)
    }
}
