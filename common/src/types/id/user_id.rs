//! ID of a user

use super::CompactUuid;
use crate::impl_compact_uuid_wrapper;
use serde::{Deserialize, Serialize};

/// ID of a user
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Decode)]
#[serde(transparent)]
pub struct UserId(CompactUuid);

impl_compact_uuid_wrapper!(UserId);
