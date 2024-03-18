use std::future::Future;

use common::types::{Id, UserId};
use sqlx::SqlitePool;

/// Trait for polymorphically running queries on different databases
pub trait DbQuery {
    /// Insert a new user
    fn insert_user<'a>(
        &'a self,
        id: &'a UserId,
    ) -> impl Future<Output = sqlx::Result<()>> + Send + '_;
}

impl DbQuery for SqlitePool {
    fn insert_user<'a>(
        &'a self,
        id: &'a UserId,
    ) -> impl Future<Output = sqlx::Result<()>> + Send + '_ {
        async move {
            let id_bytes = id.as_bytes();

            sqlx::query!(
                "--sql
                INSERT INTO users VALUES (?);
                ",
                id_bytes,
            )
            .execute(self)
            .await?;

            Ok(())
        }
    }
}
