use std::time::Duration;

use common::types::UserId;

/// Actions that can operate on the shared state
use crate::{api, state::AppState};

pub async fn send_message(state: AppState, message: String, from: UserId, to: UserId) {
    tokio::spawn(api::send_message(from, to, message.clone()));

    state.lock_store_with_repaint().insert_message(message);
}

pub async fn start_conversation(state: AppState, other_id: UserId) {
    println!("Staring a conversation with {other_id}");
    let self_id = state.ui_store.lock().self_id();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let Ok(fetched_messages) = api::fetch_unread_messages(self_id, other_id).await else {
                continue;
            };

            if fetched_messages.is_empty() {
                continue;
            }

            {
                let mut store_lock = state.lock_store_with_repaint();

                for message in fetched_messages.iter() {
                    let message = String::from_utf8_lossy(&message.content).into_owned();
                    dbg!(&message);
                    store_lock.insert_message(message);
                }
            }
            
            api::mark_received(
                self_id,
                other_id,
                fetched_messages.iter().map(|message| message.id).collect(),
            )
            .await
            .unwrap();
        }
    });
}
