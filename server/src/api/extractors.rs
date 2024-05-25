//! Additional Axum extractors

use std::{fmt::Debug, future::Future, marker::Send, pin::Pin};

use axum::{
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, BytesMut};
use common::utils;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{AppError, IntoAppResult};

/// Implement `From` `Deref` and `DerefMut` traits for a wrapper tuple struct.
macro_rules! impl_wrapper {
    ($t:ident) => {
        impl<T> From<T> for $t<T> {
            fn from(value: T) -> Self {
                Self(value)
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

/// Allows serializing and deserializing types from bytes using TryInto and TryFrom traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OctetStream<T>(pub T);

impl_wrapper!(OctetStream);

impl<S, T> FromRequest<S> for OctetStream<T>
where
    S: Send + Sync,
    T: for<'a> TryFrom<&'a [u8]>,
    for<'a> <T as TryFrom<&'a [u8]>>::Error: Debug,
{
    type Rejection = AppError;

    fn from_request<'life0, 'async_trait>(
        req: Request,
        state: &'life0 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let fut = async {
            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(|e| AppError::new(e.status(), e.body_text()))?;

            let inner = T::try_from(&bytes).with_code_and_message(
                StatusCode::BAD_REQUEST,
                "Failed to parse the request body",
            )?;

            Ok(Self(inner))
        };

        Box::pin(fut)
    }
}

impl<T> IntoResponse for OctetStream<T>
where
    for<'a> &'a [u8]: TryFrom<&'a T>,
    for<'a> <&'a [u8] as TryFrom<&'a T>>::Error: Debug,
    T: 'static,
{
    fn into_response(self) -> Response {
        (&self.0)
            .try_into()
            .with_generic_error()
            .map(|b| Bytes::copy_from_slice(b))
            .into_response()
    }
}

/// Allows serializing and deserializing types to/from the CBOR format using [`serde`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cbor<T>(pub T);

impl_wrapper!(Cbor);

impl<S, T> FromRequest<S> for Cbor<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    fn from_request<'life0, 'async_trait>(
        req: Request,
        state: &'life0 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        fn content_type_is_cbor(headers: &HeaderMap) -> bool {
            headers
                .get(header::CONTENT_TYPE)
                .and_then(|content_type| content_type.to_str().ok())
                .and_then(|content_type| content_type.parse::<mime::Mime>().ok())
                .map(|content_type| {
                    content_type.type_() == "application"
                        && (content_type.subtype() == "cbor"
                            || content_type.suffix().map_or(false, |name| name == "cbor"))
                })
                .unwrap_or(false)
        }

        let fut = async {
            if !content_type_is_cbor(req.headers()) {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Expected a request with Content-Type: application/cbor",
                ));
            }

            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(|e| AppError::new(e.status(), e.body_text()))?;

            let inner = utils::cbor::from_reader(&*bytes).with_code_and_message(
                StatusCode::BAD_REQUEST,
                "Failed to parse the request body",
            )?;

            Ok(Self(inner))
        };

        Box::pin(fut)
    }
}

impl<T> IntoResponse for Cbor<T>
where
    T: Serialize + 'static,
{
    fn into_response(self) -> Response {
        let mut buf = BytesMut::new().writer();

        let result = ciborium::into_writer(&self.0, &mut buf);
        if let Err(_) = result {
            return AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not deserialize response",
            )
            .into_response();
        };

        let bytes = buf.into_inner().freeze();
        let mut res = Body::from(bytes).into_response();
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/cbor"),
        );

        res
    }
}
