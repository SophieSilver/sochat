use crate::account::{AccountId, AccountInfo, AccountRecord};
use common::{
    forward_from_impl,
    types::{Id, UserId},
};
use reqwest::Url;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};
use std::{path::Path, sync::Arc};
use thiserror::Error;
use tokio_stream::{Stream, StreamExt};

pub use sqlx::sqlite;

// #[derive(Debug, Error)]
// pub enum StorageError {
//     #[error(transparent)]
//     Sql(#[from] sqlx::Error),
//     #[error("error while parsing a url")]
//     UrlParse(#[from] url::ParseError),
// }

// forward_from_impl!(sqlx::migrate::MigrateError => sqlx::Error => StorageError);

#[derive(Debug, Clone)]
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn with_connection_pool(pool: SqlitePool) -> sqlx::Result<Self> {
        sqlx::migrate!().run(&pool).await?;
        sqlx::query!("VACUUM;").execute(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn connect_with_filename(filename: impl AsRef<Path>) -> sqlx::Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true)
            .pragma("foreign_keys", "ON")
            .optimize_on_close(true, None)
            .journal_mode(SqliteJournalMode::Wal);

        Self::with_connection_pool(SqlitePool::connect_with(options).await?).await
    }

    pub async fn connect_in_memory() -> sqlx::Result<Self> {
        let options = SqliteConnectOptions::new()
            .in_memory(true)
            .pragma("foreign_keys", "ON");

        // because of
        // https://github.com/launchbadge/sqlx/issues/2510
        // all data is lost when creating additional connections
        // So we force a single connection.
        // This is suboptimal, however we only use in-memory pools for testing, sooo, it should be alright
        let pool = SqlitePoolOptions::new()
            .idle_timeout(None)
            .max_lifetime(None)
            .min_connections(1)
            .max_connections(1)
            .connect_with(options)
            .await?;

        Self::with_connection_pool(pool).await
    }

    pub(crate) async fn store_hub(&self, url: &Url) -> sqlx::Result<()> {
        let url = url.as_str();
        sqlx::query!(
            "--sql
            INSERT OR IGNORE INTO Hubs VALUES (?);
            ",
            url
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub(crate) async fn store_account(&self, info: &AccountInfo) -> sqlx::Result<AccountId> {
        let hub_url = info.hub_url.as_str();
        let user_id = info.user_id.as_bytes();

        let result = sqlx::query!(
            "--sql
            INSERT INTO Accounts (hub_url, user_id)
                VALUES (?, ?)
            RETURNING id;
            ",
            hub_url,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(AccountId(result.id))
    }

    pub(crate) fn load_accounts(
        &self,
    ) -> impl Stream<Item = sqlx::Result<AccountRecord>> + use<'_> //
    {
        sqlx::query!(
            "--sql
            SELECT id, hub_url, user_id AS 'user_id: UserId' FROM Accounts;
            "
        )
        .fetch(&self.pool)
        .map(|out| -> sqlx::Result<_> {
            let out = out?;

            Ok(AccountRecord {
                id: AccountId(out.id),
                info: AccountInfo {
                    hub_url: Arc::new(
                        Url::parse(&out.hub_url).map_err(|e| sqlx::Error::Decode(e.into()))?,
                    ),
                    user_id: out.user_id,
                },
            })
        })
    }
}
