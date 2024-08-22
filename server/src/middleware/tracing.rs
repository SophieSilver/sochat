//! Extension to tracing middleware

mod sealed {
    pub trait Sealed {}
}
use std::{fmt::Display, marker::PhantomData};

use axum::extract::{connect_info::MockConnectInfo, ConnectInfo};
use sealed::Sealed;
use tower_http::trace::{HttpMakeClassifier, MakeSpan, TraceLayer};
use tracing::Level;

/// An extension for [`TraceLayer`] to add connection info to trace spans
pub trait TraceLayerExt: Sealed {
    /// Helper method to add connection info to spans created by [`TraceLayer`].
    ///
    /// Behaves identically to [`TraceLayer::new_for_http`]
    fn new_for_http_with_connection_info<T>(
    ) -> TraceLayer<HttpMakeClassifier, MakeSpanWithConnectionInfo<T>>
    where
        T: Clone + Display + Send + Sync + 'static;
}

/// Similar to [`tower_http::trace::DefaultMakeSpan`] 
#[derive(Debug, Clone)]
pub struct MakeSpanWithConnectionInfo<T> {
    level: Level,
    include_headers: bool,
    _phantom: PhantomData<T>,
}

impl<T> Default for MakeSpanWithConnectionInfo<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MakeSpanWithConnectionInfo<T> {
    /// Create a new `MakeSpanWithConnectionInfo`.
    pub fn new() -> Self {
        Self {
            level: Level::DEBUG,
            include_headers: false,
            _phantom: PhantomData,
        }
    }

    /// Set the [`Level`] used for the [tracing span].
    ///
    /// Defaults to [`Level::DEBUG`].
    ///
    /// [tracing span]: https://docs.rs/tracing/latest/tracing/#spans
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Include request headers on the [`Span`].
    ///
    /// By default headers are not included.
    ///
    /// [`Span`]: tracing::Span
    pub fn include_headers(mut self, include_headers: bool) -> Self {
        self.include_headers = include_headers;
        self
    }
}

impl<B, T> MakeSpan<B> for MakeSpanWithConnectionInfo<T>
where
    T: Clone + Display + Send + Sync + 'static,
{
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        // See https://docs.rs/tower-http/latest/src/tower_http/trace/make_span.rs.html#78-112
        macro_rules! make_span {
            ($level:expr, $connection_info:expr) => {
                match ($connection_info, self.include_headers) {
                    (None, true) => {
                        make_span!(@finalize $level, headers: [?request.headers()])
                    }
                    (None, false) => {
                        make_span!(@finalize $level,)
                    }
                    (Some(ConnectInfo(info)), true) => {
                        make_span!(@finalize $level, connection: [%info] headers: [?request.headers()])
                    }
                    (Some(ConnectInfo(info)), false) => {
                        make_span!(@finalize $level, connection: [%info])
                    }
                }
            };

            (
                @finalize $level:expr,
                $(connection: [$($conn:tt)*])?
                $(headers: [$($head:tt)*])?
            ) => { {
                    tracing::span!(
                        $level,
                        "request",
                        $(from = $($conn)*,)?
                        method = %request.method(),
                        uri = %request.uri(),
                        verion = ?request.version(),
                        $(headers = $($head)*,)?
                    )
                }
            };
        }

        let connection_info = request
            .extensions()
            .get::<ConnectInfo<T>>()
            .cloned()
            .or_else(|| {
                request
                    .extensions()
                    .get::<MockConnectInfo<T>>()
                    .map(|MockConnectInfo(info)| ConnectInfo(info.clone()))
            });

        match self.level {
            Level::ERROR => make_span!(Level::ERROR, connection_info),
            Level::WARN => make_span!(Level::WARN, connection_info),
            Level::INFO => make_span!(Level::INFO, connection_info),
            Level::DEBUG => make_span!(Level::DEBUG, connection_info),
            Level::TRACE => make_span!(Level::TRACE, connection_info),
        }
    }
}

impl Sealed for TraceLayer<HttpMakeClassifier> {}

impl TraceLayerExt for TraceLayer<HttpMakeClassifier> {
    fn new_for_http_with_connection_info<T>(
    ) -> TraceLayer<HttpMakeClassifier, MakeSpanWithConnectionInfo<T>>
    where
        T: Clone + Display + Send + Sync + 'static,
    {
        let trace_layer = TraceLayer::new_for_http();
        
        trace_layer.make_span_with(MakeSpanWithConnectionInfo::<T>::new())
    }
}
