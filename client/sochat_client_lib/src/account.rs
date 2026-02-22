use std::sync::Arc;

use common::{types::UserId, utils::sql::SqlxErrorExt};
use reqwest::Url;
use tokio_stream::StreamExt;

use crate::{
    api_client::ApiClient,
    chat::{Chat, ChatData, ChatRecord},
    error::AppError,
    storage::Storage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub(crate) struct AccountId(pub i64);

#[derive(Debug, Clone)]
pub(crate) struct AccountData {
    pub hub_url: Arc<Url>,
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub(crate) struct AccountRecord {
    pub id: AccountId,
    pub data: AccountData,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub(crate) api_client: ApiClient,
    pub(crate) storage: Storage,
    pub(crate) record: AccountRecord,
}

impl Account {
    pub async fn start_new_chat(&self, other_id: UserId) -> crate::Result<Chat> {
        // TODO: some kinda invisible hello message
        // to make sure the other user actually exists
        let chat_data = ChatData {
            account_id: self.record.id,
            other_id,
        };

        let result = self.storage.store_chat(chat_data).await;

        let id = match result {
            Ok(chat_id) => chat_id,
            Err(e) if e.is_unique_violation() => {
                return Err(AppError::ChatAlreadyExists { other_id }.into())
            }
            Err(e) => return Err(e.into()),
        };

        Ok(Chat {
            account: self.clone(),
            other_id,
            id,
        })
    }

    // TODO: pagination and sorting
    pub async fn fetch_chats(&self) -> crate::Result<Vec<Chat>> {
        self.storage
            .load_chats_for_account(&self.record.id)
            .map(|record| {
                let ChatRecord {
                    id,
                    data: ChatData { other_id, .. },
                } = record?;

                Ok(Chat {
                    account: self.clone(),
                    id,
                    other_id,
                })
            })
            .collect()
            .await
    }

    
}
