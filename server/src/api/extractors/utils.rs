//! Utils for extractors

use std::fmt::Debug;

use axum::{
    body::Body,
    extract::{FromRequest, Request},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::{buf::Writer, BufMut, Bytes, BytesMut};

use crate::error::{AppError, AppResult, IntoAppResult};

/// Check if the content type matches `application/{type_name}`
fn content_type_matches(headers: &HeaderMap, type_name: &str) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .and_then(|content_type| content_type.parse::<mime::Mime>().ok())
        .map(|content_type| {
            content_type.type_() == "application"
                && (content_type.subtype() == type_name
                    || content_type
                        .suffix()
                        .map_or(false, |name| name == type_name))
        })
        .unwrap_or(false)
}

/// Error if the content type of the request does not match `application/{type_name}`
pub fn ensure_content_type_matches(headers: &HeaderMap, type_name: &str) -> AppResult<()> {
    match content_type_matches(headers, type_name) {
        true => Ok(()),
        false => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            format!(
                "Expected a request with Content-Type: application/{}",
                type_name
            ),
        )),
    }
}

/// Retrieve the request body as bytes and deserialize it using the user provided closure.
pub async fn deserialize_bytes_from_request<S, T, F, E>(
    req: Request,
    state: &S,
    f: F,
) -> AppResult<T>
where
    S: Send + Sync,
    F: FnOnce(&[u8]) -> Result<T, E>,
    E: Debug,
{
    let bytes = Bytes::from_request(req, state)
        .await
        .map_err(|e| AppError::new(e.status(), e.body_text()))?;

    let value = f(&bytes)
        .with_code_and_message(StatusCode::BAD_REQUEST, "Failed to parse the request body")?;

    Ok(value)
}

/// Serialize the provided type into response using the provided closure and add Content-Type to headers
///
/// The `Content-Type` header will be set to `application/{type_name}`
pub fn serialize_into_response<T, F, E>(value: T, f: F, type_name: &str) -> Response
where
    E: Debug,
    F: FnOnce(T, &mut Writer<BytesMut>) -> Result<(), E>,
{
    let mut buf = BytesMut::new().writer();

    let result = f(value, &mut buf).with_message("Failed to serialize response");
    if let Err(e) = result {
        return e.into_response();
    }

    let bytes = buf.into_inner().freeze();
    let mut res = Body::from(bytes).into_response();
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("application/{type_name}"))
            .expect("Constructing a HeaderValue must not fail"),
    );

    res
}
