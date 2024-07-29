//! The CBOR extractor for Axum

use std::{future::Future, pin::Pin};

use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};
use common::utils;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppError;

use super::utils::{
    deserialize_bytes_from_request, ensure_content_type_matches, serialize_into_response,
};

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
            ensure_content_type_matches(req.headers(), "cbor")?;

            let inner =
                deserialize_bytes_from_request(req, state, |bytes| utils::cbor::from_reader(bytes))
                    .await?;

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
        serialize_into_response(
            self.0,
            |value, writer| ciborium::into_writer(&value, writer),
            "cbor",
        )
    }
}
