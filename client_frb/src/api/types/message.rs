use client_service::common::types::UserId;
use flutter_rust_bridge::frb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub from: UserId,
    pub to: UserId,
    pub content: String,
}