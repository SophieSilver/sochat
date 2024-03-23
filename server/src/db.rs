use common::types::{message_id::MessageId, Id, UnreadMessage, UserId};
use futures_util::TryFutureExt;
use itertools::Itertools;
use once_cell::sync::Lazy;
use sqlx::{Execute, QueryBuilder, Sqlite, SqlitePool};
use std::{future::Future, iter};
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
    async fn insert_user(&self, id: &UserId) -> sqlx::Result<()> {
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

    async fn insert_message(
        &self,
        id: &MessageId,
        sender: &UserId,
        recipient: &UserId,
        content: &[u8],
    ) -> sqlx::Result<()> {
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

    async fn mark_messages_received(&self, ids: &[MessageId]) -> sqlx::Result<()> {
        // SQLite cannot really bind an array of things,
        // so we have to make custom queries without compile time verification

        // we are gonna send ids in batches of 1024
        // we will probably never see more than a few dozen ids here
        // but just to be on the safe side
        const BATCH_SIZE: usize = 1024;

        // I intentionally don't try to send them all at once
        // because that might cause lock contention on the database
        for batch in ids.chunks(BATCH_SIZE) {
            // dynamically create a query with as many ? placeholders as we need
            let mut query_builder = QueryBuilder::new(
                "
                UPDATE messages
                SET is_received = TRUE
                WHERE id IN (
                ",
            );
            let mut placeholders = query_builder.separated(',');

            for id in batch {
                placeholders.push_bind(id.as_bytes());
            }

            query_builder.push(");");

            let query = query_builder.build();
            query.execute(self).await?;
        }

        Ok(())
    }

    async fn fetch_unread_messages(
        &self,
        sender: &UserId,
        recipient: &UserId,
        limit: u32,
    ) -> sqlx::Result<Box<[UnreadMessage]>> {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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

        let unreceived = pool
            .fetch_unread_messages(&user1, &user2, MESSAGE_COUNT as _)
            .await?;

        for message in unreceived.iter() {
            assert_eq!(
                unreceived_pile[&message.id].as_slice(),
                &message.content[..]
            );
        }

        Ok(())
    }
}
