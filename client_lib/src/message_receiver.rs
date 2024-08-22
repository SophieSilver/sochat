use std::time::Duration;

use common::types::{api_params::FetchMessagesParams, UnreadMessage, UserId};
use tokio::{task, time::MissedTickBehavior};

use crate::server_connection::{ServerConnection, ServerConnectionError};

/// Type that receives messages from the server and calls the provided callback on new messages
#[derive(Debug)]
pub struct MessageReceiver {
    task_handle: task::JoinHandle<()>,
}

impl MessageReceiver {
    /// Create a new MessageReceiver with the provided closure as the callback
    /// 
    /// # Panics
    /// This function panics if called outside of the tokio runtime
    pub fn new<F>(connection: ServerConnection, user_id: UserId, mut callback: F) -> Self
    where
        F: FnMut(Result<UnreadMessage, ServerConnectionError>) + Send + 'static,
    {
        let task_handle = tokio::spawn(async move {
            let mut timer = tokio::time::interval(Duration::from_secs(1));
            timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                timer.tick().await;
                let result = connection
                    .fetch_messages(&FetchMessagesParams { user_id, limit: 0 })
                    .await;

                match result {
                    Ok(messages) => {
                        for m in messages {
                            callback(Ok(m))
                        }
                    }

                    Err(e) => callback(Err(e)),
                }
            }
        });

        Self { task_handle }
    }
}

impl Drop for MessageReceiver {
    fn drop(&mut self) {
        self.task_handle.abort();
    }
}
