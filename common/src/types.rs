//! Types used for sending information between the server and the client
//!
//! All types that are sent between the client and the server must be defined in this module to avoid serialization errors

pub mod id;
pub mod error;
pub mod message;

pub use error::*;
pub use id::*;
pub use message::*;
