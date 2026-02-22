use std::sync::Arc;

use common::forward_from_impl;
use reqwest::Client;
use thiserror::Error;
use tokio_stream::StreamExt;
use tracing::instrument;
use url::Url;

use crate::{
    account::{Account, AccountData, AccountRecord},
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

    /// Register a new account with a given Hub URL
    #[instrument(skip_all, fields(%hub_url), ret)]
    pub async fn register_account(&self, hub_url: Arc<Url>) -> crate::Result<Account> {
        // TODO: flesh out hubs (certificates, some info, etc.)
        self.state.storage.store_hub(&hub_url).await?;

        let api_client = ApiClient::new(Client::sochat_new()?, hub_url.clone());
        let user_id = api_client.register_user().await?;

        // FIXME: what if we die after registering the user and before saving it to storage?

        let account_data = AccountData { hub_url, user_id };
        let account_id = self.state.storage.store_account(&account_data).await?;

        let account_record = AccountRecord {
            id: account_id,
            data: account_data,
        };

        Ok(Account {
            api_client,
            record: account_record,
            storage: self.state.storage.clone(),
        })
    }

    /// Get a list of already registered accounts
    pub async fn fetch_accounts(&self) -> crate::Result<Vec<Account>> {
        self.state
            .storage
            .load_accounts()
            .map(|record| -> crate::Result<_> {
                let record = record?;
                let api_client = ApiClient::new(Client::sochat_new()?, record.data.hub_url.clone());
                let account = Account {
                    api_client,
                    record,
                    storage: self.state.storage.clone(),
                };

                Ok(account)
            })
            .collect()
            .await
    }
}
