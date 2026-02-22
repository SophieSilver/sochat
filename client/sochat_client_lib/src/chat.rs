use common::types::UserId;

use crate::account::{Account, AccountId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub(crate) struct ChatId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ChatData {
    pub account_id: AccountId,
    pub other_id: UserId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ChatRecord {
    pub id: ChatId,
    pub data: ChatData,
}

#[derive(Debug, Clone)]
pub struct Chat {
    pub(crate) account: Account,
    pub(crate) id: ChatId,
    pub(crate) other_id: UserId,
}

impl Chat {
    /// Send a single message to the other user
    pub fn send_message() {
        todo!()
    }

    /// Get a stream of incoming messages from the other user
    pub fn recv_messages() {
        todo!()
    }

    pub fn account(&self) -> &Account {
        &self.account
    }
}
