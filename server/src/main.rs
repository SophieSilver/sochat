// #![cfg_attr(debug_assertions, allow(unused))]
use server::{
    api::{self, state::AppState},
    middleware::tracing::TraceLayerExt,
};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::{net::SocketAddr, str::FromStr};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_target(true)
        .with_max_level(Level::DEBUG)
        .finish()
        .init();
    
    tracing::info!("Starting up");

    let connection_options = SqliteConnectOptions::new()
        .filename("run.db")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let db_pool = sqlx::SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!().run(&db_pool).await?;

    let app_state = AppState::new(db_pool.clone());
    let router = api::router(app_state).layer(
        tower::ServiceBuilder::new()
            .layer(TraceLayer::new_for_http_with_connection_info::<SocketAddr>()),
    );

    let app = router.into_make_service_with_connect_info::<SocketAddr>();
    let addr = SocketAddr::from_str("127.0.0.1:11800").unwrap();

    tokio::select! {
        _ = axum_server::bind(addr).serve(app) => (),
        _ = tokio::signal::ctrl_c() => ()
    };

    tracing::info!("Shutting down");
    db_pool.close().await;

    Ok(())
}
