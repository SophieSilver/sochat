use std::{fmt::Debug, hash::Hash, mem};

use crate::utils::CompactUuid;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

use super::{Id, IdSliceWrongSizeError};

// might use a different Id for the user in the future
// it seems that maybe 128 bits is too much

/// An ID that uniquely identifies a user
#[serde_as]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId {
    #[serde_as(as = "CompactUuid")]
    uuid: Uuid,
}

impl Debug for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BUF_SIZE: usize = mem::size_of::<Uuid>() * 2;
        let mut buf = [0u8; BUF_SIZE];
        let engine = base64::prelude::BASE64_URL_SAFE_NO_PAD;
        let size = engine
            .encode_slice(self.uuid.as_bytes(), &mut buf)
            .expect("Buf must be sufficiently large");

        let encoded = std::str::from_utf8(&buf[..size]).expect("Base64 should be valid UTF8");

        f.debug_tuple("MessageId").field(&encoded).finish()
    }
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
