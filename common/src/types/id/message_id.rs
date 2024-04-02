use super::CompactUuid;
use crate::impl_compact_uuid_wrapper;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId(CompactUuid);

impl_compact_uuid_wrapper!(MessageId);
