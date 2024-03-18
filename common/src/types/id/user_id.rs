use std::hash::Hash;

use crate::utils::CompactUuid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

use super::{Id, IdSliceWrongSizeError};

// might use a different Id for the user in the future
// it seems that maybe 128 bits is too much

/// An ID that uniquely identifies a user
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId {
    #[serde_as(as = "CompactUuid")]
    uuid: Uuid,
}

impl Id for UserId {
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

impl_additional_traits_for_id!(UserId);
