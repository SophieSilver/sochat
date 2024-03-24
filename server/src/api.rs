pub mod state;

use crate::{
    db::Db,
    error::{AppError, AppResult},
    extractors::{Cbor, OctetStream},
};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::StatusCode,
    routing, Router,
};
use common::types::{message_id::MessageId, Id, UnreadMessage, UserId};
use serde::Deserialize;
use smallvec::SmallVec;
use state::AppState;

#[derive(Debug, Clone, Copy, Deserialize)]
struct MessagePathParams {
    /// ID of the user initiating the request
    ///
    /// When sending messages it is the sender's ID,
    /// When receiving messages it is the recipient's ID
    pub self_id: UserId,
    /// ID of the other user in the conversation
    ///
    /// When sending messages it is the recipient's ID,
    /// When receiving messages it is the sender's ID
    pub other_id: UserId,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct LimitQueryParam {
    pub limit: u32,
}

async fn register_user(state: State<AppState>) -> AppResult<OctetStream<UserId>> {
    let new_id = UserId::generate();
    state.db().insert_user(&new_id).await?;

    Ok(new_id.into())
}

async fn send_message(
    state: State<AppState>,
    Path(MessagePathParams {
        self_id: sender_id,
        other_id: recipient_id,
    }): Path<MessagePathParams>,
    content: Bytes,
) -> AppResult<OctetStream<MessageId>> {
    let message_id = MessageId::generate();

    let result = state
        .db()
        .insert_message(&message_id, &sender_id, &recipient_id, &content)
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

async fn mark_received(
    state: State<AppState>,
    Path(MessagePathParams {
        other_id: sender_id,
        self_id: recipient_id,
    }): Path<MessagePathParams>,
    Cbor(message_ids): Cbor<SmallVec<[MessageId; 4]>>, // the most common case would only have 1 ID
) -> AppResult<StatusCode> {
    // TODO: have a limit and make it configurable
    state
        .db()
        .mark_messages_received(&sender_id, &recipient_id, &message_ids)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn fetch_messages(
    state: State<AppState>,
    Path(MessagePathParams {
        other_id: sender_id,
        self_id: recipient_id,
    }): Path<MessagePathParams>,
    limit: Option<Query<LimitQueryParam>>,
) -> AppResult<Cbor<Box<[UnreadMessage]>>> {
    // TODO: make this configurable
    const DEFAULT_LIMIT: u32 = 10;
    const MAX_LIMIT: u32 = 32;

    let limit = limit
        .map(|wrapper| wrapper.0.limit)
        .unwrap_or(DEFAULT_LIMIT)
        .min(MAX_LIMIT);

    let messages = state
        .db()
        .fetch_unread_messages(&sender_id, &recipient_id, limit)
        .await?;

    Ok(messages.into())
}

/// Create a router with all API endpoints
pub fn router(app_state: AppState) -> Router<()> {
    use routing::method_routing as method;

    Router::new()
        .route("/users", method::post(register_user))
        .route(
            "/messages/from/:self_id/to/:other_id",
            method::post(send_message),
        )
        .route(
            "/messages/from/:other_id/to/:self_id/received",
            method::post(mark_received),
        )
        .route(
            "/messages/from/:other_id/to/:self_id",
            method::get(fetch_messages),
        )
        .with_state(app_state.clone())
}
