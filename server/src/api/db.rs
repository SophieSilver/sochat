use std::{
    borrow::Cow,
    future::{Future, IntoFuture},
    iter,
};

use common::types::{message_id::MessageId, Id, UnreadMessage, UserId};
use futures_util::TryFutureExt;
use itertools::Itertools;
use once_cell::sync::Lazy;
use sqlx::{
    query::{self, Query},
    sqlite::SqliteArguments,
    QueryBuilder, Sqlite, SqlitePool,
};
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

    fn mark_messages_received(&self, ids: &[MessageId]) -> send_future!(sqlx::Result<()>);
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

    fn mark_messages_received(&self, ids: &[MessageId]) -> send_future!(sqlx::Result<()>) {
        // SQLite cannot really bind an array of things,
        // so we have to make custom queries without compile time verification

        // we are gonna send ids in batches of 32
        const CHUNK_SIZE: usize = 32;

        // pre-made query string with 32 placeholders
        static QUERY_STRING: Lazy<String> = Lazy::new(|| {
            let mut string = String::from(
                "
                UPDATE messages
                SET is_received = TRUE
                WHERE id IN (
                ",
            );

            #[allow(unstable_name_collisions)]
            let placeholders = iter::repeat('?').take(CHUNK_SIZE).intersperse(',');

            string.extend(placeholders);
            string.push_str(");");

            string
        });

        async move {
            //let scope = async_scoped::TokioScope::;
            // firing all queries at once
            let tasks = ids.chunks(CHUNK_SIZE).map(|chunk| {
                let mut query = sqlx::query::<Sqlite>(&QUERY_STRING);

                // doing that instead of directly iterating
                // because we still need to fill out empty spots with NULLs
                for i in 0..CHUNK_SIZE {
                    let id = chunk.get(i).map(|id| id.as_bytes());
                    query = query.bind(id);
                }

                // throwing away the results of the futures as we're only really interested in whether it succeeds or not
                query.execute(self).map_ok(|_| ())
            });
            
            futures_util::future::try_join_all(tasks).await?;

            Ok(())
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
