//! Types used for sending information between the server and the client
//!
//! All types that are sent between the client and the server must be defined in this module to avoid serialization errors

pub mod id;
pub mod error;

pub use error::*;
pub use id::*;
