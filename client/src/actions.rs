use crate::state::AppState;

pub async fn send_message(state: AppState, message: String) {
    println!("starting to send a message");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("sending");
    
    state
        .lock_store_with_repaint()
        .await
        .insert_message(message);
}
