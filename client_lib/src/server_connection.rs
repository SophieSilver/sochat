//! Communication with a backend server
use bytes::Bytes;
use common::{
    from_passthrough,
    types::{Id, MessageId, UnreadMessage, UserId},
};
use reqwest::Client;
use thiserror::Error;

use crate::http_utils::{
    error::{HttpError, SerializationError, StatusError},
    RequestBuilderExt, ResponseExt,
};
// TODO: switch to postcard
// TODO: adapt to new API
// TODO: unhardcode this
const SERVER_ADDR: &str = "http://127.0.0.1:11800";

#[derive(Debug, Error)]
#[error("Error while communicating with the backend")]
pub enum ServerConnectionError {
    Http(#[from] HttpError),
    Status(#[from] StatusError),
}
from_passthrough!(reqwest::Error => HttpError => ServerConnectionError);
from_passthrough!(SerializationError => HttpError => ServerConnectionError);

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
            .await?
            .filter_status_error()
            .await?;

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
            .filter_status_error()
            .await?;

        let bytes = response.bytes().await?;
        let message_id = MessageId::from_bytes(&bytes).map_err(SerializationError::from)?;

        Ok(message_id)
    }

    /// Fetch unread messages from one user to another
    pub async fn fetch_messages(
        &self,
        sender: UserId,
        recipient: UserId,
        limit: u32,
    ) -> Result<Box<[UnreadMessage]>, ServerConnectionError> //
    {
        let response = self
            .client
            .get(format!(
                "{SERVER_ADDR}/messages/from/{sender}/to/{recipient}?limit={limit}"
            ))
            .send()
            .await?
            .filter_status_error()
            .await?;

        let messages = response.cbor::<Box<[UnreadMessage]>>().await?;

        Ok(messages)
    }

    pub async fn mark_messages_received<I: AsRef<[MessageId]>>(
        &self,
        sender: UserId,
        recipient: UserId,
        ids: I,
    ) -> Result<(), ServerConnectionError> //
    {
        let ids = ids.as_ref();

        let _response = self
            .client
            .post(format!(
                "{SERVER_ADDR}/messages/from/{sender}/to/{recipient}/received"
            ))
            .cbor(ids)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        Ok(())
    }
}
