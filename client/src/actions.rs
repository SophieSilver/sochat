use crate::state::AppState;

pub async fn send_message(state: AppState, message: String) {
    state
        .lock_store_with_repaint()
        .await
        .insert_message(message);
}
