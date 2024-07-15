//! Communication with a backend server
use bytes::Bytes;
use common::types::{ApiError, Id, MessageId, UserId};
use reqwest::Client;
use thiserror::Error;

use crate::http_utils::{
    error::{CborError, ResponseError, StatusError},
    ResponseExt,
};

// TODO: unhardcode this
const SERVER_ADDR: &str = "127.0.0.1:11800";

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SerializationError {
    Cbor(#[from] CborError),
    Id(#[from] common::types::id::IdSliceWrongSizeError),
}

#[derive(Debug, Error)]
#[error("error while communicating with the server")]
pub enum ServerConnectionError {
    Request(#[from] reqwest::Error),
    Serialize(#[from] SerializationError),
    Status(#[from] StatusError),
}

impl From<ResponseError> for ServerConnectionError {
    fn from(value: ResponseError) -> Self {
        match value {
            ResponseError::Request(e) => Self::from(e),
            ResponseError::Deserialize(e) => {
                Self::Serialize(SerializationError::Cbor(CborError::from(e)))
            }
        }
    }
}

/// Connection to a backend server
///
/// Used to send and receive messages, as well as register new users
#[derive(Debug, Clone)]
pub struct ServerConnection {
    client: Client,
}

impl ServerConnection {
    /// Create a new connection from a supplied HTTP client
    pub const fn new(client: Client) -> Self {
        Self { client }
    }

    pub const fn http_client(&self) -> &Client {
        &self.client
    }

    // TODO: implement retries if no connection
    // TODO: implement timeouts
    /// Register a new user on the server and return its ID
    pub async fn register_user(&self) -> Result<UserId, ServerConnectionError> {
        let response = self
            .client
            .post(format!("{SERVER_ADDR}/users"))
            .send()
            .await?;

        let response = response.filter_api_error().await?;

        let bytes = response.bytes().await?;
        let id = UserId::from_bytes(&bytes).map_err(SerializationError::from)?;

        Ok(id)
    }

    /// Send a message from one user to another
    pub async fn send_message<M: Into<Bytes>>(
        &self,
        sender: UserId,
        recipient: UserId,
        content: M,
    ) -> Result<MessageId, ServerConnectionError> //
    {
        let response = self
            .client
            .post(format!(
                "{SERVER_ADDR}/messages/from/{sender}/to/{recipient}"
            ))
            .body(content.into())
            .send()
            .await?
            .filter_api_error()
            .await?;

        let bytes = response.bytes().await?;
        let message_id = MessageId::from_bytes(&bytes).map_err(SerializationError::from)?;

        Ok(message_id)
    }
}
