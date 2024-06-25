use std::sync::{Arc, Mutex};

pub use common::types::MessageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_id: MessageId,
    pub text: String,
}
