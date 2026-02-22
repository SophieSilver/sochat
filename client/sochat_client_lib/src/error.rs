use std::error::Error as StdError;

use common::{
    cbor::CborError,
    forward_from_impl,
    types::{ApiError, IdParseError, UserId},
};
use thiserror::Error;

use crate::chat::{ChatData, ChatId};

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

/// Error in application logic
///
/// This is an error kind that is expected to occur the most
/// and the application is supposed to be most prepared in dealing with it
#[derive(Debug, Error)]
pub enum AppError {
    #[error("chat with user {other_id} already exists")]
    ChatAlreadyExists { other_id: UserId },
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("HTTP error")]
    Http(#[source] reqwest::Error),
    #[error("serialization error")]
    Serialization(#[source] SerializationError),
    #[error("storage error")]
    Storage(#[from] sqlx::Error),
    #[error("hub API error")]
    Api(#[from] ApiError),
    #[error(transparent)]
    Application(#[from ]AppError),
}

// manual impl so that we have impls for all variants of serialization errors
impl<T> From<T> for ErrorKind
where
    SerializationError: From<T>,
{
    fn from(value: T) -> Self {
        let error = SerializationError::from(value);

        Self::Serialization(error)
    }
}

// manual impl because we want to turn decode errors into serialization errors
impl From<reqwest::Error> for ErrorKind {
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

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    pub fn from_kind(kind: ErrorKind) -> Self {
        kind.into()
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    // note: this can't be a trait as that would break the blanket impl
    pub fn into_kind(self) -> ErrorKind {
        *self.kind
    }
}

impl<T> From<T> for Error
where
    ErrorKind: From<T>,
{
    fn from(value: T) -> Self {
        Self {
            kind: Box::new(value.into()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
