use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse, Json};
use common::types::ApiError;
use thiserror::Error;

// TODO: box errors for smaller footprint

/// Generic error type with a message and a source
#[derive(Debug, Clone)]
pub struct AppError {
    error: ApiError,
    code: StatusCode,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            error: ApiError {
                error: message.into(),
            },
            code,
        }
    }

    pub fn generic() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
    }

    pub fn error(&self) -> &ApiError {
        &self.error
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }
}

impl Default for AppError {
    fn default() -> Self {
        Self::generic()
    }
}

impl<E: Error> From<E> for AppError {
    fn from(_: E) -> Self {
        // Don't wanna accidentally leak the specifics of the error
        Self::generic()
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self.error)).into_response()
    }
}

/// An extension for Result to convert them into AppResults
pub trait IntoAppResult {
    type Out;
    /// If the result is an error, convert it into a generic AppResult
    fn with_generic_error(self) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult by using the
    /// error's `Display` implementation as the message and the provided status code.
    fn with_code(self, code: StatusCode) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult by using the
    /// custom message and `500 INTERNAL SERVER ERROR` as the code.
    fn with_message(self, message: &str) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult by using the
    /// custom status code and message.
    fn with_code_and_message(self, code: StatusCode, message: &str) -> AppResult<Self::Out>;
}

impl<T, E> IntoAppResult for Result<T, E>
{
    type Out = T;

    fn with_generic_error(self) -> AppResult<Self::Out> {
        self.map_err(|_| AppError::generic())
    }

    fn with_code(self, code: StatusCode) -> AppResult<Self::Out> {
        self.map_err(|_| AppError {
            code,
            ..Default::default()
        })
    }

    fn with_message(self, message: &str) -> AppResult<Self::Out> {
        self.with_code_and_message(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    fn with_code_and_message(self, code: StatusCode, message: &str) -> AppResult<Self::Out> {
        self.map_err(|_| AppError::new(code, message))
    }
}
