//! Various extensions to the HTTP client.
//!
//! These extensions provide the default configurations for the http client,
//! as well as additional serialization formats for request bodies

use common::{types::ApiError, utils::cbor};
use error::{CborSerializeError, ResponseError, StatusError};
use reqwest::{header, tls, Certificate, Client, ClientBuilder, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, time::Duration};

// ERRORS
pub mod error;

// SEALED TRAIT
mod private {
    use reqwest::{Client, RequestBuilder, Response};

    pub trait Sealed {}
    impl Sealed for Client {}
    impl Sealed for RequestBuilder {}
    impl Sealed for Response {}
}

use private::Sealed;

// CONSTANTS
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
const USER_AGENT: &str = "SoChatClient/0.0";


// EXTENSION TRAITS
/// Extension trait for [`reqwest::Client`]
pub trait ClientExt: Sealed + Sized {
    /// Create a new HTTP client configured for sochat with built in root certificate
    ///
    /// # Errors
    /// see [`reqwest::ClientBuilder::build#errors`]
    fn sochat_new() -> reqwest::Result<Self>;

    /// Create a new HTTP client configured for sochat with supplied certificates
    ///
    /// # Errors
    /// see [`reqwest::ClientBuilder::build#errors`]
    fn sochat_with_certs<C: IntoIterator<Item = Certificate>>(certs: C) -> reqwest::Result<Self>;
}

/// Extension trait for [`reqwest::RequestBuilder`]
pub trait RequestBuilderExt: Sealed + Sized {
    /// Send a CBOR body
    ///
    /// # Errors
    /// This method fails when serializing the payload with CBOR fails
    fn cbor<T: Serialize + ?Sized>(self, cbor: &T) -> Result<Self, CborSerializeError>;
}

/// Extension trait for [`reqwest::Response`]
pub trait ResponseExt: Sealed + Sized {
    /// Try to deserialize the response body as CBOR
    ///
    /// # Errors
    /// This method fails when:
    /// * Fetching the response body fails
    /// * Trying to deserialize the response body with CBOR fails
    fn cbor<T: DeserializeOwned>(self) -> impl Future<Output = Result<T, ResponseError>> + Send;

    /// Check if the status code indicates an error, if so return the error, otherwise return the original response.
    /// 
    /// # Returns
    /// This function returns a double result.
    /// 
    /// The outer result indicates whether fetching and deserializing the response body was successful.
    /// 
    /// The inner result indicates whether the request succeded or not.
    /// 
    /// Thus, this function returns:
    /// - `Ok(Ok(self))` if the response's status isn't an error.
    /// - `Ok(Err(api_error))` if the response's status is an error.
    /// - `Err(response_error)` if the response's status is an error but 
    /// fetching or deserializing the response body has failed.
    fn filter_status_error(
        self,
    ) -> impl Future<Output = Result<Self, StatusError>> + Send;
}

// IMPLS
impl ClientExt for Client {
    fn sochat_new() -> reqwest::Result<Self> {
        default_builder().tls_built_in_root_certs(true).build()
    }

    fn sochat_with_certs<C: IntoIterator<Item = Certificate>>(certs: C) -> reqwest::Result<Self> {
        let mut builder = default_builder().tls_built_in_root_certs(false);

        for c in certs.into_iter() {
            builder = builder.add_root_certificate(c);
        }

        builder.build()
    }
}

impl RequestBuilderExt for RequestBuilder {
    fn cbor<T: Serialize + ?Sized>(mut self, cbor: &T) -> Result<Self, CborSerializeError> {
        // TODO: use Bytes here for cheaper cloning
        let mut buf = Vec::<u8>::new();

        ciborium::into_writer(cbor, &mut buf)?;

        self = self.header(header::CONTENT_TYPE, "application/cbor");

        Ok(self.body(buf))
    }
}

impl ResponseExt for Response {
    async fn cbor<T: DeserializeOwned>(self) -> Result<T, ResponseError> {
        let bytes = self.bytes().await?;

        Ok(cbor::from_reader(&bytes as &[u8])?)
    }
    // TODO: store the status code somewhere
    async fn filter_status_error(self) -> Result<Self, StatusError>  {
        let status = self.status();
        let is_error = status.is_client_error() || status.is_server_error();

        if !is_error {
            return Ok(self);
        }

        let error = self.cbor::<ApiError>().await;
        Err(StatusError {
            status,
            source: error,
        })
    }
}

// HELPERS

fn default_builder() -> ClientBuilder {
    Client::builder()
        .use_rustls_tls()
        .min_tls_version(tls::Version::TLS_1_3)
        .http2_prior_knowledge()
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .user_agent(USER_AGENT)
}
