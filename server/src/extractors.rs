use ::core::{future::Future, marker::Send, pin::Pin};

use axum::{
    body::{Body, Bytes},
    extract::{FromRef, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::error::{AppError, IntoAppResult};

/// Allows serializing and deserializing types from bytes using TryInto and TryFrom traits
pub struct OctetStream<T>(pub T);

impl<T> From<T> for OctetStream<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<S, T> FromRequest<S> for OctetStream<T>
where
    S: Send + Sync,
    T: for<'a> TryFrom<&'a [u8]>,
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
                .with_code_and_message(StatusCode::BAD_REQUEST, "Invalid request body")?;

            let inner = T::try_from(&bytes)
                .with_code_and_message(StatusCode::BAD_REQUEST, "Invalid request body")?;

            Ok(Self(inner))
        };

        Box::pin(fut)
    }
}

impl<T> IntoResponse for OctetStream<T>
where
    for<'a> &'a [u8]: TryFrom<&'a T>,
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
