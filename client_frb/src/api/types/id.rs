use crate::api::types::marker::Opaque;
use client_lib::common::types::Id;
pub use client_lib::common::types::{MessageId, UserId};
use flutter_rust_bridge::frb;
use std::str::FromStr;

/// Trait for extending Id types for use with [`flutter_rust_bridge`]
pub trait IdExt: Id + ToString + FromStr
where
    Self::Err: std::error::Error + Send + Sync + 'static,
{
    // renamed to avoid conflict with ToString::to_string
    #[frb(sync, name = "to_string")]
    fn to_string_dart(&self) -> String {
        self.to_string()
    }

    /// Parse ID from a string
    #[frb(sync, positional)]
    fn parse(value: String) -> anyhow::Result<Self> {
        Ok(Self::from_str(&value)?)
    }

    #[frb(sync, getter)]
    fn hash_code(&self) -> i64 {
        unimplemented!("Please avoid hashing IDs. \n\
Due to FRB limitations equality operator cannot be overriden, therefore a hashCode cannot be defined correctly");

        // const SIZE: usize = mem::size_of::<i64>();

        // let bytes = self.as_bytes();
        // let mut hash_code_bytes = [0u8; SIZE];

        // for (index, &byte) in bytes.iter().enumerate() {
        //     // XORing with the byte at that location
        //     hash_code_bytes[index % SIZE] ^= byte;
        // }

        // i64::from_le_bytes(hash_code_bytes)
    }

    /// Use instead of == operator due to FRB limitations
    #[frb(sync, positional)]
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

macro_rules! extend_id {
    ($type_name: ident, $mirror_name: ident) => {
        #[frb(
                            mirror($type_name),
                        //     dart_code =
                        // "
                        //     @override
                        //     bool operator ==(Object other) {
                        //         if (this.runtimeType != other.runtimeType) {
                        //             return false;
                        //         }

                        //         return this.equals(other as dynamic);
                        //     }
                        // "
                        )]
        struct $mirror_name {
            _opq: Opaque,
        }

        impl IdExt for $type_name {}
    };
}

extend_id!(UserId, _UserIdMirror);
extend_id!(MessageId, _MessageIdMirror);
