//! Statically checked ID types

use std::hash::Hash;

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// A trait with common methods for IDs
///
/// Contains common serialization methods for ID types
pub trait Id: Sized + Copy + Eq + Hash + Serialize + DeserializeOwned {
    /// Create a new ID
    fn generate() -> Self;

    /// Serialize the ID as a byte slice.
    fn as_bytes(&self) -> &[u8];

    /// Deserializes an ID from a byte slice
    ///
    /// # Errors
    /// Returns an error when the provided byte slice is not the correct length.
    ///
    /// ## Note
    /// The correct length is implementation defined and is free to change.
    ///
    /// As such this method should only be used to deserialize byte strings obtained via the `as_bytes` method
    /// of the same struct
    fn from_bytes(bytes: &[u8]) -> Result<Self, IdSliceWrongSizeError>;
}

/// Automatically implement `AsRef<[u8]>`, `TryFrom<&[u8]>`, as well as `From<&T>` for `&[u8]`
#[macro_export]
macro_rules! impl_additional_traits_for_id {
    ($t: ty) => {
        impl AsRef<[u8]> for $t {
            fn as_ref(&self) -> &[u8] {
                $crate::types::id::Id::as_bytes(self)
            }
        }

        impl<'a> From<&'a $t> for &'a [u8] {
            fn from(value: &'a $t) -> Self {
                $crate::types::id::Id::as_bytes(value)
            }
        }

        impl TryFrom<&[u8]> for $t {
            type Error = $crate::types::id::IdSliceWrongSizeError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                <Self as $crate::types::id::Id>::from_bytes(value)
            }
        }

        $crate::impl_sqlx_decode_from_bytes!($t);
    };
}

/// An error type that indicates that the user tried to convert a byte slice into an ID, but the slice was the wrong size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Tried to create an ID from a byte slice of the wrong size")]
pub struct IdSliceWrongSizeError;

pub mod compact_uuid;
pub use compact_uuid::*;

pub mod message_id;
pub mod user_id;
pub use user_id::*;
pub use message_id::*;
