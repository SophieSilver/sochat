//! Additional Axum extractors

/// Implement `From`, `AsRef`, `Borrow`, `Deref` and `DerefMut` traits for a wrapper tuple struct.
macro_rules! impl_wrapper {
    ($t:ident) => {
        impl<T> From<T> for $t<T> {
            fn from(value: T) -> Self {
                Self(value)
            }
        }

        impl<T> AsRef<T> for $t<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> std::borrow::Borrow<T> for $t<T> {
            fn borrow(&self) -> &T {
                &self.0
            }
        }

        impl<T> std::ops::Deref for $t<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $t<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
pub mod utils;
pub mod cbor;
pub mod octet_stream;
pub mod postcard;

pub use cbor::Cbor;
pub use octet_stream::OctetStream;
