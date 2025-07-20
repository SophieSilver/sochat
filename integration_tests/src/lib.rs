// this stuff is only usedul for tests, so we don't compile anything when not testing
use std::net::TcpListener;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

pub struct AbortGuard {
    handle: tokio::task::AbortHandle,
}

impl Drop for AbortGuard {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .try_init();
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
