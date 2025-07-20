//! Communication with a backend server
use std::sync::Arc;

use common::types::{
    api_params::{FetchMessagesParams, MarkReceivedParams, SendMessageParams},
    Id, MessageId, UnreadMessage, UserId,
};
use reqwest::{Client, Url};

use crate::{
    error::SerializationError,
    http_utils::{RequestBuilderExt, ResponseExt},
};

/// Implementation of the Sochat Hub API client
///
/// Used to send and receive messages, as well as register new users
///
/// This struct is cheaply cloneable
#[derive(Debug, Clone)]
pub struct ApiClient {
    http: Client,
    hub_url: Arc<Url>,
}

impl ApiClient {
    /// Create a new connection from a supplied HTTP client
    pub fn new(http_client: Client, hub_url: impl Into<Arc<Url>>) -> Self {
        Self {
            http: http_client,
            hub_url: hub_url.into(),
        }
    }

    pub const fn http_client(&self) -> &Client {
        &self.http
    }

    // TODO: implement timeouts
    /// Register a new user on the server and return its ID
    pub async fn register_user(&self) -> crate::Result<UserId> {
        let response = self
            .http
            .post(self.hub_url.join("register_user").expect("url parses"))
            .send()
            .await?
            .filter_status_error()
            .await?;

        let bytes = response.bytes().await?;
        let id = UserId::from_bytes(&bytes)?;

        Ok(id)
    }

    /// Send a message from one user to another
    pub async fn send_message(&self, params: &SendMessageParams) -> crate::Result<MessageId> {
        let response = self
            .http
            .post(self.hub_url.join("send_message").expect("url parses"))
            .postcard(params)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        let bytes = response.bytes().await?;
        let message_id = MessageId::from_bytes(&bytes).map_err(SerializationError::from)?;

        Ok(message_id)
    }

    /// Fetch unread messages
    pub async fn fetch_messages(
        &self,
        params: &FetchMessagesParams,
    ) -> crate::Result<Vec<UnreadMessage>> {
        let response = self
            .http
            .get(self.hub_url.join("fetch_messages").expect("url parses"))
            .postcard(params)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        let messages = response.postcard::<Vec<UnreadMessage>>().await?;

        Ok(messages)
    }

    pub async fn mark_messages_received(&self, params: &MarkReceivedParams) -> crate::Result<()> {
        let _response = self
            .http
            .post(self.hub_url.join("mark_received").expect("url parses"))
            .postcard(params)?
            .send()
            .await?
            .filter_status_error()
            .await?;

        Ok(())
    }
}
