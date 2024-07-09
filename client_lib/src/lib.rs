//! Library crate containing core functionality for the SoChat client, such as communicating with the backend servers,
//! storing user data on disk, and encryption.

#![forbid(missing_docs)]
pub use common;

pub mod types;
pub mod http_utils;
pub mod server_connection;