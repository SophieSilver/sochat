//! Postcard extractor for axum

use ::core::{future::Future, marker::Send, pin::Pin};

use axum::{
    body::Body,
    extract::{FromRequest, Request},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use bytes::{BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppError;

use super::utils::{deserialize_bytes_from_request, ensure_content_type_matches};

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
            ensure_content_type_matches(req.headers(), "postcard")?;

            let inner =
                deserialize_bytes_from_request(req, state, |bytes| postcard::from_bytes(bytes))
                    .await?;

            Ok(Self(inner))
        };

        Box::pin(fut)
    }
}

impl<T> IntoResponse for Postcard<T>
where
    T: Serialize + 'static,
{
    fn into_response(self) -> axum::response::Response {
        let mut buf = BytesMut::new().writer();

        let result = postcard::to_io(&self.0, &mut buf);
        if let Err(_) = result {
            return AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serialize response",
            )
            .into_response();
        }

        let bytes = buf.into_inner().freeze();
        let mut res = Body::from(bytes).into_response();
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/postcard"),
        );

        res
    }
}
