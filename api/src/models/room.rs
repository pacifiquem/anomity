use async_session::chrono::FixedOffset;
use serde::Serialize;
use sqlx::types::chrono::DateTime as SqlxDateTime;
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: i32,
    pub name: Option<String>,
    pub user_id: i32,
    pub created_at: SqlxDateTime<FixedOffset>,
}

#[derive(Serialize, Debug)]
pub struct RoomParticipants {
    pub room_id: i32,
    pub user_id: Uuid,
}
