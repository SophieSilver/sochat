use std::{
    fmt::{Debug, Display},
    hash::Hash,
    mem,
    str::FromStr,
};

use crate::utils::BytesOrBase64Uuid;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;
use uuid::Uuid;

use super::{Id, IdSliceWrongSizeError};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Could not parse ID")]
pub enum CompactUuidParseError {
    IdSliceWrongSize(#[from] IdSliceWrongSizeError),
    Base64Error(#[from] base64::DecodeError),
}

impl From<base64::DecodeSliceError> for CompactUuidParseError {
    fn from(value: base64::DecodeSliceError) -> Self {
        match value {
            base64::DecodeSliceError::DecodeError(decode_error) => Self::Base64Error(decode_error),
            base64::DecodeSliceError::OutputSliceTooSmall => {
                Self::IdSliceWrongSize(IdSliceWrongSizeError)
            }
        }
    }
}

/// A UUIDv7 that is serialized with base64 instead of hex to make it's representation more compact
#[serde_as]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompactUuid {
    #[serde_as(as = "BytesOrBase64Uuid")]
    uuid: Uuid,
}

impl Display for CompactUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BUF_SIZE: usize = mem::size_of::<Uuid>() * 2; // being a bit generous here, it should be about 1.333 times the size in memory
        let mut buf = [0u8; BUF_SIZE];
        let engine = base64::prelude::BASE64_URL_SAFE_NO_PAD;
        let size = engine
            .encode_slice(self.uuid.as_bytes(), &mut buf)
            .expect("Buf must be sufficiently large");

        let encoded = std::str::from_utf8(&buf[..size]).expect("Base64 should be valid UTF8");

        f.write_str(encoded)
    }
}

impl FromStr for CompactUuid {
    type Err = CompactUuidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const BUF_SIZE: usize = mem::size_of::<Uuid>();
        let mut buf = [0u8; BUF_SIZE];
        let engine = base64::prelude::BASE64_URL_SAFE_NO_PAD;
        let size = engine.decode_slice(s, &mut buf)?;
        let bytes = &buf[..size];
        Ok(Self::from_bytes(bytes)?)
    }
}

impl Debug for CompactUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Uuid")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl Id for CompactUuid {
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

impl_additional_traits_for_id!(CompactUuid);

/// Implement some traits on a type that wraps a [`CompactUuid`].
///
/// # Requirements
/// The type must be a tuple struct with the only field being a [`CompactUuid`].
///
/// The type must also implement all supertraits of [`Id`].
///
/// # Implemented traits
/// * [`Display`]
/// * [`Debug`]
/// * [`Id`]
/// * [`FromStr`]
/// * [`AsRef<\[u8\]>`]
/// * [`From<&Self>`] for `&[u8]`
/// * [`TryFrom<&\[u8\]>`]
/// * [`sqlx::Decode`]
///
/// # Examples
/// ```
/// #[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// #[serde(transparent)]
/// pub struct UuidWrapper(CompactUuid);
///
/// impl_compact_uuid_wrapper!(UuidWrapper);
/// ```
#[macro_export]
macro_rules! impl_compact_uuid_wrapper {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&format_args!("{self}"))
                    .finish()
            }
        }

        impl std::str::FromStr for $t {
            type Err = <$crate::types::id::CompactUuid as std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }

        impl $crate::types::id::Id for $t {
            fn generate() -> Self {
                Self($crate::types::id::CompactUuid::generate())
            }

            fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            fn from_bytes(bytes: &[u8]) -> Result<Self, $crate::types::id::IdSliceWrongSizeError> {
                Ok(Self($crate::types::id::CompactUuid::from_bytes(bytes)?))
            }
        }

        $crate::impl_additional_traits_for_id!($t);
    };
}
