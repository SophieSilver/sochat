use std::{error::Error as StdError, fmt::Display};

use common::{
    cbor::CborError, forward_from_impl, types::{id, ApiError}
};
use reqwest::StatusCode;
use thiserror::Error;

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("error while serializing request body")]
    Serialization(#[from] SerializationError),
    #[error("error while sending request")]
    Request(#[from] reqwest::Error),
}

forward_from_impl!(CborError => SerializationError => HttpError);
forward_from_impl!(postcard::Error => SerializationError => HttpError);
forward_from_impl!(id::IdParseError => SerializationError => HttpError);

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SerializationError {
    Cbor(#[from] CborError),
    Postcard(#[from] postcard::Error),
    Id(#[from] id::IdParseError),
}

#[derive(Debug)]
pub struct StatusError {
    pub status: StatusCode,
    pub source: Result<ApiError, HttpError>,
}

impl Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // reqwest::Error::source()
        write!(f, "backend server returned an error code {}", self.status)
    }
}

impl StdError for StatusError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        let source = match &self.source {
            Ok(api_error) => api_error as &dyn StdError,
            Err(response_error) => response_error as &dyn StdError,
        };

        Some(source)
    }
}
