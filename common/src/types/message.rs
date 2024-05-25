//! types of messages that are exchanged between the client and the server

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::BytesOrBase64;

use super::message_id::MessageId;

/// Message that has not yet been received
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnreadMessage {
    /// ID of the message
    pub id: MessageId,
    /// The actual content of the message
    #[serde_as(as = "BytesOrBase64")]
    pub content: Box<[u8]>,
}