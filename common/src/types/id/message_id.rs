use std::hash::Hash;

use crate::utils::CompactUuid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

// everything is literally the same as user id

use super::{Id, IdSliceWrongSizeError};

/// An ID that uniquely identifies a message
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageId {
    #[serde_as(as = "CompactUuid")]
    uuid: Uuid,
}

impl Id for MessageId {
    fn generate() -> Self {
        Self {
            uuid: Uuid::now_v7(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        self.uuid.as_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, IdSliceWrongSizeError> {
        Ok(Self {
            uuid: Uuid::from_slice(bytes).map_err(|_| IdSliceWrongSizeError)?,
        })
    }
}

impl_additional_traits_for_id!(MessageId);
