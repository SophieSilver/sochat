//! The CBOR extractor for Axum

use std::{future::Future, pin::Pin};

use axum::{
    body::Body,
    extract::{FromRequest, Request},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, Bytes, BytesMut};
use common::utils;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{AppError, IntoAppResult};

use super::utils::content_type_matches;

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
        let fut = async {
            if !content_type_matches(req.headers(), "cbor") {
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
                "Failed to serialize response",
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
