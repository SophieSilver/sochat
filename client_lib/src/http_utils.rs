use common::utils::cbor;
use reqwest::{tls, Certificate, Client, ClientBuilder, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, io, time::Duration};
use thiserror::Error;

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

// ERRORS

#[derive(Debug, Error)]
#[error(transparent)]
pub struct CborSerializeError(#[from] ciborium::ser::Error<io::Error>);

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ResponseCborError {
    Request(#[from] reqwest::Error),
    Cbor(#[from] ciborium::de::Error<io::Error>),
}

// EXTENSION TRAITS

pub trait ClientExt: Sealed + Sized {
    fn sochat_new() -> reqwest::Result<Self>;

    fn sochat_with_certs<C: IntoIterator<Item = Certificate>>(certs: C) -> reqwest::Result<Self>;
}

pub trait RequestBuilderExt: Sealed + Sized {
    fn cbor<T: Serialize + ?Sized>(self, cbor: &T) -> Result<Self, CborSerializeError>;
}

pub trait ResponseExt: Sealed + Sized {
    fn cbor<T: DeserializeOwned>(self)
        -> impl Future<Output = Result<T, ResponseCborError>> + Send;
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
    fn cbor<T: Serialize + ?Sized>(self, cbor: &T) -> Result<Self, CborSerializeError> {
        let mut buf = Vec::<u8>::new();

        ciborium::into_writer(cbor, &mut buf)?;

        Ok(self.body(buf))
    }
}

impl ResponseExt for Response {
    async fn cbor<T: DeserializeOwned>(self) -> Result<T, ResponseCborError> {
        let bytes = self.bytes().await?;

        Ok(cbor::from_reader(&bytes as &[u8])?)
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
