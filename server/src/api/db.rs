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
                WHERE sender_id = ?
                    AND recipient_id = ?
                    AND is_received = FALSE
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

#[cfg(test)]
mod tests {
    use std::{collections::{HashMap, HashSet}, time::Duration};

    use futures_util::future::try_join_all;

    use super::*;

    #[sqlx::test]
    async fn insert_user_test(pool: SqlitePool) -> sqlx::Result<()> {
        let user_id = UserId::generate();

        pool.insert_user(&user_id).await?;
        assert_eq!(
            sqlx::query!(
                "--sql
            SELECT * FROM users;
            "
            )
            .fetch_one(&pool)
            .await?
            .id,
            user_id.as_bytes()
        );

        // can't insert twice
        assert!(pool.insert_user(&user_id).await.is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn insert_message_test(pool: SqlitePool) -> sqlx::Result<()> {
        let user1 = UserId::generate();
        let user2 = UserId::generate();
        pool.insert_user(&user1).await?;
        pool.insert_user(&user2).await?;

        let message_id = MessageId::generate();
        let content = b"Hi, how are you doing";

        pool.insert_message(&message_id, &user1, &user2, content)
            .await?;

        let record = sqlx::query!("SELECT * FROM messages")
            .fetch_one(&pool)
            .await?;

        let values = (
            &record.id[..],
            &record.sender_id[..],
            &record.recipient_id[..],
            &record.content[..],
            record.is_received == 1,
        );

        assert_eq!(
            values,
            (
                message_id.as_bytes(),
                user1.as_bytes(),
                user2.as_bytes(),
                &content[..],
                false
            )
        );

        // inserting message with invalid ids fails
        assert!(pool
            .insert_message(&MessageId::generate(), &user1, &UserId::generate(), content)
            .await
            .is_err());

        assert!(pool
            .insert_message(&MessageId::generate(), &UserId::generate(), &user2, content)
            .await
            .is_err());

        // duplicate insert fails
        assert!(pool
            .insert_message(&message_id, &user1, &user2, content)
            .await
            .is_err());

        Ok(())
    }

    #[sqlx::test]
    async fn fetch_unread_messages_test(pool: SqlitePool) -> sqlx::Result<()> {
        let user1 = UserId::generate();
        let user2 = UserId::generate();
        let id1 = MessageId::generate();
        let id2 = MessageId::generate();
        let content1 = b"Hello there!!!";
        let content2 = b"Well, goodbye then";

        pool.insert_user(&user1).await?;
        pool.insert_user(&user2).await?;
        pool.insert_message(&id1, &user1, &user2, content1).await?;
        pool.insert_message(&id2, &user1, &user2, content2).await?;

        let id1_bytes = id1.as_bytes();
        sqlx::query!(
            "UPDATE messages SET is_received = TRUE WHERE id = ?",
            id1_bytes
        )
        .execute(&pool)
        .await?;

        let messages = pool.fetch_unread_messages(&user1, &user2, 100).await?;

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, id2);
        assert_eq!(&messages[0].content[..], &content2[..]);

        Ok(())
    }

    #[sqlx::test]
    async fn mark_received_test(pool: SqlitePool) -> sqlx::Result<()> {
        const MESSAGE_COUNT: usize = 500;
        // how many messages to mark as received
        const RECEIVED_COUNT: usize = 300;

        let user1 = UserId::generate();
        let user2 = UserId::generate();
        pool.insert_user(&user1).await?;
        pool.insert_user(&user2).await?;

        let messages = (0..MESSAGE_COUNT)
            .map(|i| {
                (
                    MessageId::generate(),
                    format!("Message number {i}").into_bytes(),
                )
            })
            .collect_vec();

        for (id, content) in &messages {
            pool.insert_message(id, &user1, &user2, content).await?;
        }
        
        // mark the first several messages as received
        let (received_pile, unreceved_pile) = messages.split_at(RECEIVED_COUNT);
        let unreceived_pile = unreceved_pile
            .into_iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        
        let received_ids = received_pile.iter().map(|(id, _)| id.clone()).collect_vec();

        pool.mark_messages_received(&received_ids).await?;
        
        let unreceived = pool.fetch_unread_messages(&user1, &user2, MESSAGE_COUNT as _).await?;
        
        for message in unreceived.iter() {
            assert_eq!(unreceived_pile[&message.id].as_slice(), &message.content[..]);
        }

        Ok(())
    }
}
