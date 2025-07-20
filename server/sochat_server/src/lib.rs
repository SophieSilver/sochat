//! Server library crate
#![warn(missing_docs)]

use std::{io, net::{SocketAddr, TcpListener}, time::Duration};

use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;

use crate::{api::state::AppState, middleware::tracing::TraceLayerExt};

pub mod api;
pub mod db;
pub mod error;
pub mod middleware;

// TODO: a more proper server implementation

/// Run the server with the provided database connection and socket address
pub async fn run(db: SqlitePool, tcp: TcpListener) -> anyhow::Result<()> {
    tracing::info!("Starting server");

    sqlx::migrate!().run(&db).await?;

    let app_state = AppState::new(db.clone());
    let router = api::router(app_state).layer(
        tower::ServiceBuilder::new()
            .layer(TraceLayer::new_for_http_with_connection_info::<SocketAddr>()),
    );

    let app = router.into_make_service_with_connect_info::<SocketAddr>();
    let handle = axum_server::Handle::new();
    let server = axum_server::from_tcp(tcp).handle(handle.clone());

    tokio::spawn(async move {
        let res = tokio::signal::ctrl_c().await;
        if let Err(e) = res {
            let e = anyhow::Error::from(e);

            tracing::error!("Error waiting for the CTRL+C handler: {e:#}");
        }

        tracing::info!("Starting graceful shutdown");
        handle.graceful_shutdown(Some(Duration::from_secs(5)));
    });

    server.serve(app).await?;
    db.close().await;

    tracing::info!("Server shut down");

    Ok(())
}
