//! Utils for extractors

use axum::http::{header, HeaderMap};

/// Check if the content type matches `application/{type_name}`
pub fn content_type_matches(headers: &HeaderMap, type_name: &str) -> bool {
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