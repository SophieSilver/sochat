//! The octet stream extractor for Axum

use crate::error::{AppError, IntoAppResult};
use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use std::{fmt::Debug, future::Future, pin::Pin};

use super::utils::deserialize_bytes_from_request;

/// Allows serializing and deserializing types from bytes using TryInto and TryFrom traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OctetStream<T>(pub T);

impl_wrapper!(OctetStream);

impl<S, T> FromRequest<S> for OctetStream<T>
where
    S: Send + Sync,
    for<'a> T: TryFrom<&'a [u8]>,
    for<'a> <T as TryFrom<&'a [u8]>>::Error: Debug,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let inner = deserialize_bytes_from_request(req, state, |bytes| {
            // mapping error to a String solves complicated lifetime issues
            T::try_from(bytes).map_err(|e| format!("{e:?}"))
        })
        .await?;

        Ok(Self(inner))
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
            .map(Bytes::copy_from_slice)
            .into_response()
    }
}
