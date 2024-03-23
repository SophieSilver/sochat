pub mod state;

use crate::{db::Db, error::AppResult, extractors::OctetStream};
use axum::{extract::State, routing, Router};
use common::types::{Id, UserId};
use state::AppState;

async fn register_user(state: State<AppState>) -> AppResult<OctetStream<UserId>> {
    let new_id = UserId::generate();
    state.db().insert_user(&new_id).await?;

    Ok(new_id.into())
}

/// Create a router with all API endpoints
pub fn router(app_state: AppState) -> Router<()> {
    use routing::method_routing as method;

    Router::new().route(
        "/users",
        method::post(register_user).with_state(app_state.clone()),
    )
}
