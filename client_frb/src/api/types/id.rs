use std::mem;

pub use client_service::common::types::id::{UserId as UserIdRaw, MessageId as MessageIdRaw, Id};
use flutter_rust_bridge::frb;


macro_rules! wrap_id {
    ($name: ident, $raw_name: ident) => {
        #[frb(dart_code = "
            @override
            bool operator ==(Object other) {
                if (other.runtimeType != this.runtimeType) {
                    return false;
                }
                return this.equals(other: other as dynamic);}     // weird curly to get around frb bug
        ")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name($raw_name);

        impl $name {
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

        impl From<$name> for $raw_name {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<$raw_name> for $name {
            fn from(value: $raw_name) -> Self {
                Self(value)
            }
        }
    };
}

wrap_id!(UserId, UserIdRaw);
wrap_id!(MessageId, MessageIdRaw);
