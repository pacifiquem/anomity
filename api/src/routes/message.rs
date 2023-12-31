use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::{Claims, Room};
use crate::AppState;

pub async fn get_all_rooms(state: State<Arc<AppState>>, claims: Claims) -> Json<Vec<Room>> {
    Json(Room::get_rooms(claims.sub, &state.pg_pool).await)
}

#[derive(Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
}

pub async fn create_room(
    state: State<Arc<AppState>>,
    claims: Claims,
    Json(req): Json<NewRoom>,
) -> Json<Room> {
    let room_result = Room::create_room(claims.sub, Some(req.name), &state.pg_pool).await;

    Json(room_result)
}

pub async fn delete_room(
    state: State<Arc<AppState>>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Json<Option<Room>> {
    let deleted_room = Room::delete_room(id, claims.sub, &state.pg_pool).await;

    Json(deleted_room)
}
