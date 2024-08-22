//! Communication with a backend server
use common::{
    from_passthrough,
    types::{
        api_params::{FetchMessagesParams, MarkReceivedParams, SendMessageParams},
        Id, MessageId, UnreadMessage, UserId,
    },
};
use reqwest::Client;
use thiserror::Error;

use crate::http_utils::{
    error::{HttpError, SerializationError, StatusError},
    RequestBuilderExt, ResponseExt,
};

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
/// 
/// This struct is cheaply cloneable
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
            .post(format!("{SERVER_ADDR}/register_user"))
            .send()
            .await?
            .filter_status_error()
            .await?;

        let bytes = response.bytes().await?;
        let id = UserId::from_bytes(&bytes).map_err(SerializationError::from)?;

        Ok(id)
    }

    /// Send a message from one user to another
    pub async fn send_message(
        &self,
        params: &SendMessageParams,
    ) -> Result<MessageId, ServerConnectionError> //
    {
        let response = self
            .client
            .post(format!("{SERVER_ADDR}/send_message"))
            .postcard(params)?
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
        params: &FetchMessagesParams,
    ) -> Result<Box<[UnreadMessage]>, ServerConnectionError> //
    {
        let response = self
            .client
            .get(format!("{SERVER_ADDR}/fetch_messages"))
            .postcard(params)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        let messages = response.cbor::<Box<[UnreadMessage]>>().await?;

        Ok(messages)
    }

    pub async fn mark_messages_received(
        &self,
        params: &MarkReceivedParams,
    ) -> Result<(), ServerConnectionError> //
    {
        let _response = self
            .client
            .post(format!("{SERVER_ADDR}/mark_received"))
            .postcard(params)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        Ok(())
    }
}
