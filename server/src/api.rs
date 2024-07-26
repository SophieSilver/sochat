//! Module that defines API endpoints for the server

pub mod extractors;
pub mod state;

use crate::{
    db::Db,
    error::{AppError, AppResult},
};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::StatusCode,
    routing, Router,
};
use common::types::{
    api_params::{FetchMessagesParams, MarkReceivedParams, SendMessageParams},
    message_id::MessageId,
    Id, UnreadMessage, UserId,
};
use extractors::{Cbor, OctetStream};
use serde::Deserialize;
use smallvec::SmallVec;
use state::AppState;
use tracing::instrument;

#[instrument(skip_all, ret)]
async fn register_user(state: State<AppState>) -> AppResult<OctetStream<UserId>> {
    tracing::info!("enter");
    let new_id = UserId::generate();
    state.db().insert_user(&new_id).await?;

    Ok(new_id.into())
}

#[instrument(skip_all, ret)]
async fn send_message(
    state: State<AppState>,
    Cbor(SendMessageParams {
        user_id: sender_id,
        recipient_id,
        content,
    }): Cbor<SendMessageParams>,
) -> AppResult<OctetStream<MessageId>> //
{
    tracing::info!("enter");
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

#[instrument[skip_all, fields(message_count = message_ids.len())]]
async fn mark_received(
    state: State<AppState>,
    Cbor(MarkReceivedParams {
        user_id: sender_id,
        message_ids,
    }): Cbor<MarkReceivedParams>,
) -> AppResult<StatusCode> {
    tracing::info!("enter");
    // TODO: have a limit and make it configurable
    state
        .db()
        .mark_messages_received(&sender_id, &message_ids)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip_all, fields(limit))]
async fn fetch_messages(
    state: State<AppState>,
    Cbor(FetchMessagesParams {
        user_id: recipient_id,
        limit,
    }): Cbor<FetchMessagesParams>,
) -> AppResult<Cbor<Box<[UnreadMessage]>>> {
    tracing::info!("enter");
    // TODO: make this configurable
    const DEFAULT_LIMIT: u32 = 16;
    const MAX_LIMIT: u32 = 32;

    let limit = if limit != 0 {
        limit.min(MAX_LIMIT)
    } else {
        DEFAULT_LIMIT
    };

    let messages = state
        .db()
        .fetch_unreceived_messages(&recipient_id, limit)
        .await?;

    tracing::info!(count = messages.len(), "Return");

    Ok(messages.into())
}

/// Create a router with all API endpoints
pub fn router(app_state: AppState) -> Router<()> {
    use routing::method_routing as method;

    Router::new()
        .route("/register_user", method::post(register_user))
        .route("/send_message", method::post(send_message))
        .route("/fetch_messages", method::get(fetch_messages))
        .route("/mark_received", method::post(mark_received))
        .with_state(app_state.clone())
}
