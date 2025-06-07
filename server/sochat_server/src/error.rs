//! A module for reporting errors in the server's API

use std::{borrow::Cow, error::Error, fmt::Debug};

use axum::{http::StatusCode, response::IntoResponse, Json};
use common::types::ApiError;

/// Generic error type with a message and a status code.
#[derive(Debug, Clone)]
pub struct AppError {
    error: ApiError,
    code: StatusCode,
}

impl AppError {
    /// Create a new instance of the Error by specifying the error code and the message
    pub fn new(code: StatusCode, message: impl Into<Cow<'static, str>>) -> Self {
        let message = message.into();
        tracing::error!(%code, error_message = ?message, "AppError");

        Self {
            error: ApiError { error: message },
            code,
        }
    }

    /// Create an error with a generic error message and code `500 Internal Server Error`
    pub fn generic() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
    }

    /// Get the underlying error
    pub fn error(&self) -> &ApiError {
        &self.error
    }

    /// Get the status code
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
    fn from(e: E) -> Self {
        tracing::error!(cause = ?e, "Returning AppError");

        // Don't wanna accidentally leak the specifics of the error
        Self::generic()
    }
}

/// A type alias for a [`Result<T, AppError>`]
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // TODO: change format
        (self.code, Json(self.error)).into_response()
    }
}

/// An extension for to convert [`Result`]s them into [`AppResult`]s
pub trait IntoAppResult {
    /// The type of the success variant of the result
    type Out;
    
    /// If the result is an error, convert it into a generic AppResult
    fn with_generic_error(self) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult with a generic error message
    /// and the provided status code.
    fn with_code(self, code: StatusCode) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult by using the
    /// custom message and `500 INTERNAL SERVER ERROR` as the code.
    fn with_message(self, message: impl Into<Cow<'static, str>>) -> AppResult<Self::Out>;

    /// If the result is an error, convert the error into an AppResult by using the
    /// custom status code and message.
    fn with_code_and_message(
        self,
        code: StatusCode,
        message: impl Into<Cow<'static, str>>,
    ) -> AppResult<Self::Out>;
}

impl<T, E> IntoAppResult for Result<T, E>
where
    E: Debug,
{
    type Out = T;

    fn with_generic_error(self) -> AppResult<Self::Out> {
        self.map_err(|e| {
            tracing::error!(cause = ?e, "Returning AppError");
            AppError::generic()
        })
    }

    fn with_code(self, code: StatusCode) -> AppResult<Self::Out> {
        self.map_err(|e| {
            tracing::error!(cause = ?e, "Returning AppError");
            AppError {
                code,
                ..Default::default()
            }
        })
    }

    fn with_message(self, message: impl Into<Cow<'static, str>>) -> AppResult<Self::Out> {
        self.with_code_and_message(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    fn with_code_and_message(
        self,
        code: StatusCode,
        message: impl Into<Cow<'static, str>>,
    ) -> AppResult<Self::Out> {
        self.map_err(|e| {
            tracing::error!(cause = ?e, "Returning AppError");
            AppError::new(code, message)
        })
    }
}
