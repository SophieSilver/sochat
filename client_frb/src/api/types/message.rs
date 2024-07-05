use client_service::common::types::{MessageId, UserId};
use flutter_rust_bridge::frb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: MessageId,
    pub from: UserId,
    pub to: UserId,
    pub content: String,
}