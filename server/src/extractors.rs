use std::{
    cell::RefCell,
    future::Future,
    io::Read,
    marker::Send,
    ops::{Deref, DerefMut},
    pin::Pin,
};

use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::{BufMut, BytesMut};
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
pub struct OctetStream<T>(pub T);

impl_wrapper!(OctetStream);

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

/// Allows serializing and deserializing types to/from the CBOR format using [`serde`]
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
        const SCRATCH_SIZE: usize = 64 * 1024;

        thread_local! {
            static SCRATCH_BUFFER: RefCell<Box<[u8]>> = RefCell::new(vec![0; SCRATCH_SIZE].into_boxed_slice());
        }

        let fut = async {
            let bytes = Bytes::from_request(req, state)
                .await
                .with_code_and_message(StatusCode::BAD_REQUEST, "Invalid request body")?;

            let inner = SCRATCH_BUFFER
                .with_borrow_mut(|mut buf| ciborium::from_reader_with_buffer(&bytes[..], &mut buf))
                .with_code_and_message(StatusCode::BAD_REQUEST, "Invalid request body")?;

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

        ciborium::into_writer(&self.0, &mut buf).expect("BytesMut writer is infallible");

        buf.into_inner().freeze().into_response()
    }
}
