use std::sync::Arc;

use axum::extract::State;

use crate::models::{Claims, Room};
use crate::AppState;

pub async fn get_all_rooms(state: State<Arc<AppState>>, claims: Claims) -> Vec<Room> {
    Room::get_rooms(claims.sub, &state.pg_pool).await
}
