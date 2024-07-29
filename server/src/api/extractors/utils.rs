//! Utils for extractors

use axum::http::{header, HeaderMap, StatusCode};

use crate::error::{AppError, AppResult};

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
