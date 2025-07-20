// #![cfg_attr(debug_assertions, allow(unused))]
use sochat_server::{
    api::{self, state::AppState},
    middleware::tracing::TraceLayerExt,
};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
};
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
        .pragma("foreign_keys", "ON")
        .journal_mode(SqliteJournalMode::Wal);

    let db_pool = sqlx::SqlitePool::connect_with(connection_options).await?;

    let addr = SocketAddr::from_str("127.0.0.1:11800").unwrap();
    let tcp = TcpListener::bind(addr)?;
    sochat_server::run(db_pool, tcp).await?;
    Ok(())
}
