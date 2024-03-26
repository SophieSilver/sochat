// #![cfg_attr(debug_assertions, allow(unused))]
use std::{
    net::SocketAddr,
    str::FromStr,
};
use server::api::{self, state::AppState};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_target(true)
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let connection_options = SqliteConnectOptions::new()
        .filename("run.db")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let db_pool = sqlx::SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!().run(&db_pool).await?;

    let app_state = AppState::new(db_pool.clone());
    let router = api::router(app_state)
        .layer(tower::ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let app = router.into_make_service_with_connect_info::<SocketAddr>();
    let addr = SocketAddr::from_str("127.0.0.1:11800").unwrap();

    tokio::select! {
        _ = axum_server::bind(addr).serve(app) => (),
        _ = tokio::signal::ctrl_c() => ()
    };

    println!("exiting");
    db_pool.close().await;

    Ok(())
}
