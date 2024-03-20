use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::BytesOrBase64;

use super::message_id::MessageId;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnreadMessage {
    pub id: MessageId,
    #[serde_as(as = "BytesOrBase64")]
    pub content: Box<[u8]>,
}