#![cfg_attr(debug_assertions, allow(unused))]

use std::net::ToSocketAddrs;

use common::utils::CompactUuid;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use server::api::{self, state::AppState};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection_options = SqliteConnectOptions::new()
        .filename("run.db")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);
    let db_pool = sqlx::SqlitePool::connect_with(connection_options).await?;

    sqlx::migrate!().run(&db_pool).await?;

    let app_state = AppState::new(db_pool.clone());

    tokio::select! {
    _ = axum_server::bind("127.0.0.1:11800".to_socket_addrs().unwrap().next().unwrap())
        .serve(api::router(app_state).into_make_service()) => (),
    _ = tokio::signal::ctrl_c() => ()
    };
    
    println!("exiting");
    db_pool.close().await;

    Ok(())
}
