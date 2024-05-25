//! ID of a message

use super::CompactUuid;
use crate::impl_compact_uuid_wrapper;
use serde::{Deserialize, Serialize};

/// ID of a message
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(CompactUuid);

impl_compact_uuid_wrapper!(MessageId);
