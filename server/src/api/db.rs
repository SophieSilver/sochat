use std::{borrow::Cow, future::Future};

use common::types::{message_id::MessageId, Id, UserId};
use sqlx::SqlitePool;

/// Shortcut for `impl Future<Output = t> + Send`
macro_rules! send_future {
    ($t:ty) => {
        impl Future<Output = $t> + Send
    };
    ($t:ty, $l:lifetime) => {
        impl Future<Output = $t> + Send + $l
    }
}

/// Trait for polymorphically running queries on different databases
pub trait Db {
    /// Insert a new user
    fn insert_user(&self, id: &UserId) -> send_future!(sqlx::Result<()>);
    fn insert_message(
        &self,
        id: &MessageId,
        from: &UserId,
        to: &UserId,
        content: &[u8],
    ) -> send_future!(sqlx::Result<()>);
}

impl Db for SqlitePool {
    fn insert_user(&self, id: &UserId) -> send_future!(sqlx::Result<()>) {
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

    fn insert_message(
        &self,
        id: &MessageId,
        from: &UserId,
        to: &UserId,
        content: &[u8],
    ) -> send_future!(sqlx::Result<()>) {
        async move {
            let id = id.as_bytes();
            let from = from.as_bytes();
            let to = to.as_bytes();

            sqlx::query!(
                "--sql
                INSERT INTO messages
                    (id, sender_id, recipient_id, content, is_received)
                    VALUES (?, ?, ?, ?, FALSE);
                ",
                id,
                from,
                to,
                content,
            )
            .execute(self)
            .await?;

            Ok(())
        }
    }
}
