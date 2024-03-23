pub mod state;

use crate::{
    db::Db,
    error::{AppError, AppResult},
    extractors::OctetStream,
};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    routing, Router,
};
use common::types::{message_id::MessageId, Id, UserId};
use serde::Deserialize;
use state::AppState;

#[derive(Debug, Clone, Copy, Deserialize)]
struct MessagePathParams {
    pub user_id: UserId,
    pub other_id: UserId,
}

async fn register_user(state: State<AppState>) -> AppResult<OctetStream<UserId>> {
    let new_id = UserId::generate();
    state.db().insert_user(&new_id).await?;

    Ok(new_id.into())
}

async fn send_message(
    state: State<AppState>,
    Path(MessagePathParams { user_id, other_id }): Path<MessagePathParams>,
    content: Bytes,
) -> AppResult<OctetStream<MessageId>> {
    let message_id = MessageId::generate();

    let result = state
        .db()
        .insert_message(&message_id, &user_id, &other_id, &content)
        .await;

    match result {
        Ok(_) => Ok(message_id.into()),
        Err(sqlx::Error::Database(e)) if e.is_foreign_key_violation() => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "User with that ID does not exist",
        )),
        Err(_) => Err(AppError::generic()),
    }
}

/// Create a router with all API endpoints
pub fn router(app_state: AppState) -> Router<()> {
    use routing::method_routing as method;

    Router::new()
        .route("/users", method::post(register_user))
        .route(
            "/messages/from/:user_id/to/:other_id",
            method::post(send_message),
        )
        .with_state(app_state.clone())
}
