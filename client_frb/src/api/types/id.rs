use std::{any::Any, mem};

use super::marker::Opaque;
use client_service::common::types::Id;
pub use client_service::common::types::{MessageId, UserId};
use flutter_rust_bridge::frb;

/// Trait for extending Id types for use with [`flutter_rust_bridge`]
pub trait IdExt: Id + ToString {
    // renamed to avoid conflict with ToString::to_string
    #[frb(sync, name = "to_string")]
    fn to_string_dart(&self) -> String {
        self.to_string()
    }

    #[frb(sync, getter)]
    fn hash_code(&self) -> i64 {
        const SIZE: usize = mem::size_of::<i64>();

        let bytes = self.as_bytes();
        let mut hash_code_bytes = [0u8; SIZE];

        for (index, &byte) in bytes.iter().enumerate() {
            // XORing with the byte at that location
            hash_code_bytes[index % SIZE] ^= byte;
        }

        i64::from_le_bytes(hash_code_bytes)
    }

    /// For internal use in the operator == implementation, use == instead of this method
    #[frb(sync, ignore)]
    fn equals(self, other: Self) -> bool {
        self == other
    }
}

macro_rules! extend_id {
    ($type_name: ident, $mirror_name: ident) => {
        #[frb(
            mirror($type_name),
            dart_code =
        "
            @override
            bool operator ==(Object other) {
                if (this.runtimeType != other.runtimeType) {
                    return false;
                }
                
                return this.equals(other: other as dynamic);
            }
        "
        )]
        struct $mirror_name {
            _opq: Opaque,
        }

        impl IdExt for $type_name {
            #[frb(sync)]
            fn equals(self, other: Self) -> bool {
                self == other
            }
        }
    };
}

extend_id!(UserId, _UserIdMirror);
extend_id!(MessageId, _MessageIdMirror);
