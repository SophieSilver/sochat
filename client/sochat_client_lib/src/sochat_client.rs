use std::sync::Arc;

use common::forward_from_impl;
use reqwest::Client;
use thiserror::Error;
use url::Url;

use crate::{
    account::{Account, AccountInfo, AccountRecord},
    api_client::ApiClient,
    http_utils::ClientExt,
    storage::Storage,
};

// TODO: pool ApiClients

#[derive(Debug, Clone)]
struct SochatClientState {
    storage: Storage,
}

impl SochatClientState {}

#[derive(Debug, Clone)]
pub struct SochatClient {
    state: Arc<SochatClientState>,
}

impl SochatClient {
    pub fn new(storage: Storage) -> Self {
        Self {
            state: Arc::new(SochatClientState { storage }),
        }
    }

    pub async fn register_account(&self, hub_url: Arc<Url>) -> crate::Result<Account> {
        // TODO: flesh out hubs (certificates, some info, etc.)
        self.state.storage.store_hub(&hub_url).await?;

        let api_client = ApiClient::new(Client::sochat_new()?, hub_url.clone());
        let user_id = api_client.register_user().await?;

        // FIXME: what if we die after registering the user and before saving it to storage?

        let account_info = AccountInfo { hub_url, user_id };
        let account_id = self.state.storage.store_account(&account_info).await?;

        let account_record = AccountRecord {
            id: account_id,
            info: account_info,
        };

        Ok(Account {
            api_client,
            record: account_record,
        })
    }
}
