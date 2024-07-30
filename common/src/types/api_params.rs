//! Parameters that are sent with requests to the server

const SMALL_VEC_SIZE: usize = 4;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::{MessageId, UserId};

// TODO: pass user_id using auth headers instead

/// Parameters for `send_message` method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageParams {
    /// ID of the user initiating the request
    pub user_id: UserId,
    /// ID of the recipient of the message
    pub recipient_id: UserId,
    /// Content of the message
    pub content: Bytes,
}

/// Parameters for `mark_received` method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkReceivedParams {
    /// ID of the user initiating the request
    /// 
    /// i.e. the recipient
    pub user_id: UserId,
    /// Message IDs to mark received
    pub message_ids: SmallVec<[MessageId; SMALL_VEC_SIZE]>,
}

/// Parameters for `fetch_messages` method
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FetchMessagesParams {
    /// ID of the user initiating the request
    /// 
    /// i.e. the recipient
    pub user_id: UserId,
    /// Maximum amount of messages to fetch
    /// 
    /// `0` means no limit
    pub limit: u32,
}
