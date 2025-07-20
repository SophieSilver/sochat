use std::sync::Arc;

use common::types::UserId;
use reqwest::Url;

use crate::api_client::ApiClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Decode, sqlx::Encode)]
pub(crate) struct AccountId(pub i64);

#[derive(Debug, Clone)]
pub(crate) struct AccountInfo {
    pub hub_url: Arc<Url>,
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub(crate) struct AccountRecord {
    pub id: AccountId,
    pub info: AccountInfo,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub(crate) api_client: ApiClient,
    pub(crate) record: AccountRecord,
}

impl Account {}
