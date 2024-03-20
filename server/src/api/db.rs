use std::{
    borrow::Cow,
    future::{Future, IntoFuture},
};

use common::types::{message_id::MessageId, Id, UnreadMessage, UserId};
use sqlx::SqlitePool;
use tokio_stream::StreamExt;

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
    fn insert_user(&self, id: &UserId) -> send_future!(sqlx::Result<()>);
    fn insert_message(
        &self,
        id: &MessageId,
        sender: &UserId,
        recipient: &UserId,
        content: &[u8],
    ) -> send_future!(sqlx::Result<()>);

    fn mark_message_received(&self, ids: &MessageId) -> send_future!(sqlx::Result<()>);
    fn fetch_unread_messages(
        &self,
        sender: &UserId,
        recipient: &UserId,
        limit: u32,
    ) -> send_future!(sqlx::Result<Box<[UnreadMessage]>>);
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
        sender: &UserId,
        recipient: &UserId,
        content: &[u8],
    ) -> send_future!(sqlx::Result<()>) {
        async move {
            let id = id.as_bytes();
            let sender = sender.as_bytes();
            let recipient = recipient.as_bytes();

            sqlx::query!(
                "--sql
                INSERT INTO messages
                    (id, sender_id, recipient_id, content, is_received)
                VALUES (?, ?, ?, ?, FALSE);
                ",
                id,
                sender,
                recipient,
                content,
            )
            .execute(self)
            .await?;

            Ok(())
        }
    }

    // TODO: make this optimized for bulk marking
    fn mark_message_received(&self, id: &MessageId) -> send_future!(sqlx::Result<()>) {
        async move {
            let id = id.as_bytes();

            sqlx::query!(
                "--sql
                UPDATE messages
                SET is_received = TRUE
                WHERE id = ?;
                ",
                id
            )
            .execute(self)
            .await?;

            todo!()
        }
    }

    fn fetch_unread_messages(
        &self,
        sender: &UserId,
        recipient: &UserId,
        limit: u32,
    ) -> send_future!(sqlx::Result<Box<[UnreadMessage]>>) {
        async move {
            let sender = sender.as_bytes();
            let recipient = recipient.as_bytes();

            sqlx::query!(
                "--sql
                SELECT id, content FROM messages
                WHERE sender_id = ? AND recipient_id = ?
                ORDER BY id ASC     -- we can do that, because message ids are UUIDv7s
                LIMIT ?;
                ",
                sender,
                recipient,
                limit,
            )
            .fetch(self)
            .map(|result| {
                result.and_then(|record| {
                    Ok(UnreadMessage {
                        id: MessageId::try_from(&record.id[..])
                            .map_err(|e| sqlx::Error::Decode(e.into()))?,
                        content: record.content.into_boxed_slice(),
                    })
                })
            })
            .collect()
            .await
        }
    }
}
