use common::types::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub sender_id: UserId,
    pub recipient_id: UserId,
    pub content: String,
}
