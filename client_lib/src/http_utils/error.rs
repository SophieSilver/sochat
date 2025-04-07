use std::{error::Error as StdError, fmt::Display};

use common::{
    cbor::CborError, from_passthrough, types::{id, ApiError}
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

from_passthrough!(CborError => SerializationError => HttpError);
from_passthrough!(postcard::Error => SerializationError => HttpError);
from_passthrough!(id::IdSliceWrongSizeError => SerializationError => HttpError);

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SerializationError {
    Cbor(#[from] CborError),
    Postcard(#[from] postcard::Error),
    Id(#[from] id::IdSliceWrongSizeError),
}

#[derive(Debug)]
pub struct StatusError {
    pub status: StatusCode,
    pub source: Result<ApiError, HttpError>,
}

impl Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
