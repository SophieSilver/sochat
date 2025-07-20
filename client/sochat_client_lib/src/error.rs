use std::error::Error as StdError;

use common::{cbor::CborError, forward_from_impl, types::{ApiError, IdParseError}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("error serializing JSON")]
    Json(#[from] serde_json::Error),
    #[error("error serializing CBOR")]
    Cbor(#[from] CborError),
    #[error("error serilizing Postcard")]
    Postcard(#[from] postcard::Error),
    #[error(transparent)]
    Id(#[from] IdParseError),

    #[error("unknown serialization error")]
    Unknown(#[source] Box<dyn StdError + Send + Sync + 'static>),
}

impl SerializationError {
    /// Attempt to downcast a [`reqwest::Error`] into [`SerializationError::Json`]
    fn json_from_reqwest(e: &reqwest::Error) -> Option<Self> {
        let json_error = e.source()?.downcast_ref::<serde_json::Error>()?;
        // can't clone the fucking thing, so we just get the message
        let json_error = <serde_json::Error as serde::de::Error>::custom(json_error);

        Some(Self::Json(json_error))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error")]
    Http(#[source] reqwest::Error),
    #[error("serialization error")]
    Serialization(#[from] SerializationError),
    #[error("storage error")]
    Storage(#[from] sqlx::Error),
    #[error("hub API returned an error")]
    Api(#[from] ApiError),
}

forward_from_impl!(serde_json::Error => SerializationError => Error);
forward_from_impl!(CborError => SerializationError => Error);
forward_from_impl!(postcard::Error => SerializationError => Error);
forward_from_impl!(IdParseError => SerializationError => Error);

// manual impl because we want to turn decode errors into serialization errors
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_decode() {
            Self::Serialization(
                SerializationError::json_from_reqwest(&value)
                    .unwrap_or_else(|| SerializationError::Unknown(Box::new(value))),
            )
        } else {
            Self::Http(value)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
