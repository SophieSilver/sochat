use client_lib::common::types::{MessageId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: MessageId,
    pub from: UserId,
    pub to: UserId,
    pub content: String,
}