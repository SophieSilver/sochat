// this stuff is only usedul for tests, so we don't compile anything when not testing
use std::net::TcpListener;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

pub struct AbortGuard {
    handle: tokio::task::AbortHandle,
}

impl Drop for AbortGuard {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

pub fn spawn_test_server(tcp: TcpListener) -> AbortGuard {
    let fut = async move {
        let db_options = SqliteConnectOptions::new()
            .in_memory(true)
            .pragma("foreign_keys", "ON")
            .journal_mode(SqliteJournalMode::Wal);

        // in memory pools must be limited to a single connection,
        // they're broken otherwise
        let db = SqlitePoolOptions::new()
            .min_connections(1)
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(db_options)
            .await?;

        sochat_server::run(db, tcp).await
    };

    let handle = tokio::spawn(fut).abort_handle();

    AbortGuard { handle }
}
