//! Library crate containing core functionality for the SoChat client, such as communicating with the backend servers,
//! storing user data on disk, and encryption.

pub use common;
pub use reqwest;

pub mod account;
pub mod api_client;
pub mod chat;
pub mod http_utils;
pub mod message_receiver;
pub mod storage;

pub mod error;
pub use error::Error;
pub use error::Result;

pub mod sochat_client;
pub use sochat_client::SochatClient;

// TODO: unhardcode this
const SERVER_ADDR: &str = "http://127.0.0.1:11800";

