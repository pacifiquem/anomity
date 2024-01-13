use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::{Claims, Message, Room};
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
    Json(Room::create_room(claims.sub, Some(req.name), &state.pg_pool).await)
}

pub async fn delete_room(
    state: State<Arc<AppState>>,
    claims: Claims,
    Path(id): Path<i32>,
) -> Json<Option<Room>> {
    Json(Room::delete_room(id, claims.sub, &state.pg_pool).await)
}

pub async fn get_messages_by_room(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Json<Vec<Message>> {
    Json(Message::get_by_room_id(id, &state.pg_pool).await)
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub room_id: i32,
    pub message: String,
}

pub async fn create_message(
    state: State<Arc<AppState>>,
    claims: Claims,
    Json(req): Json<NewMessage>,
) -> Json<Message> {
    Json(Message::create(req.room_id, claims.sub, &req.message, &state.pg_pool).await)
}
