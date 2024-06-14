use std::{borrow::Borrow, mem};

pub use client_service::common::types::id::{Id, MessageId, UserId};
use flutter_rust_bridge::frb;

macro_rules! wrap_id {
    ($dart_name: ident, $name: ident) => {
        // we do this as dart code, because the operator doesn't take Self,
        // it takes Object that we're supposed to downcast and there is
        // currently
        #[frb(dart_code = "
            @override
            bool operator ==(Object other) {
                // using runtimeType instead of is operator so that it's easier to write the macro
                if (other.runtimeType != this.runtimeType) {
                    return false;
                }
                return this.equals(other: other as dynamic);
            }
        ")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $dart_name($name);

        impl $dart_name {
            #[frb(sync)]
            pub fn to_string(&self) -> String {
                format!("{}", self.0)
            }

            #[frb(sync)]
            pub fn equals(&self, other: &Self) -> bool {
                self == other
            }

            #[frb(sync, getter)]
            pub fn hash_code(&self) -> i64 {
                const SIZE: usize = mem::size_of::<i64>();

                let bytes = self.0.as_bytes();
                let mut hash_code_bytes = [0u8; SIZE];

                for (index, &byte) in bytes.iter().enumerate() {
                    // XORing with the byte at that location
                    hash_code_bytes[index % SIZE] ^= byte;
                }

                i64::from_le_bytes(hash_code_bytes)
            }
        }

        impl From<$dart_name> for $name {
            fn from(value: $dart_name) -> Self {
                value.0
            }
        }

        impl From<$name> for $dart_name {
            fn from(value: $name) -> Self {
                Self(value)
            }
        }

        impl AsRef<$name> for $dart_name {
            fn as_ref(&self) -> &$name {
                &self.0
            }
        }

        impl Borrow<$name> for $dart_name {
            fn borrow(&self) -> &$name {
                &self.0
            }
        }
    };
}

wrap_id!(DartUserId, UserId);
wrap_id!(DartMessageId, MessageId);
