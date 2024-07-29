//! Postcard extractor for axum

use ::core::{future::Future, marker::Send, pin::Pin};

use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::{
    api::extractors::utils::content_type_matches,
    error::{AppError, IntoAppResult},
};

/// Allows serializing and deserializing types to/from the Postcard format using [`serde`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Postcard<T>(pub T);
impl_wrapper!(Postcard);

impl<S, T> FromRequest<S> for Postcard<T>
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
            if !content_type_matches(req.headers(), "postcard") {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Expected a request with Content-Type: application/postcard",
                ));
            }

            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(|e| AppError::new(e.status(), e.body_text()))?;

            let inner = postcard::from_bytes::<T>(&bytes).with_code_and_message(
                StatusCode::BAD_REQUEST,
                "Failed to parse the request body",
            )?;

            Ok(Self(inner))
        };

        Box::pin(fut)
    }
}


