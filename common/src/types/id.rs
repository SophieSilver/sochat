use std::hash::Hash;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A trait with common methods for IDs
///
/// Contains common serialization methods for
pub trait Id: Sized + Clone + Eq + Hash + Serialize + for<'de> Deserialize<'de> {
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
macro_rules! impl_additional_traits_for_id {
    ($t: ty) => {
        impl AsRef<[u8]> for $t {
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }

        impl<'a> From<&'a $t> for &'a [u8] {
            fn from(value: &'a $t) -> Self {
                value.as_bytes()
            }
        }

        impl TryFrom<&[u8]> for $t {
            type Error = IdSliceWrongSizeError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Self::from_bytes(value)
            }
        }

        impl<'r, DB> sqlx::Decode<'r, DB> for $t
        where
            DB: sqlx::Database,
            for<'a> &'a [u8]: sqlx::Decode<'a, DB>,
        {
            fn decode(
                value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let value = <&[u8] as sqlx::Decode<DB>>::decode(value)?;

                Ok(Self::from_bytes(value)?)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Tried to create an ID from a byte slice of the wrong size")]
pub struct IdSliceWrongSizeError;

pub mod message_id;
pub mod user_id;
pub use user_id::*;
