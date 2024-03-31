use crate::state::AppState;

pub async fn send_message(state: AppState, message: String) {
    state.lock().await.insert_message(message);
}
