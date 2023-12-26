use async_session::chrono::FixedOffset;
use sqlx::types::chrono::DateTime as SqlxDateTime;
use uuid::Uuid;

pub struct Room {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: SqlxDateTime<FixedOffset>,
}

pub struct RoomParticipants {
    pub room_id: i32,
    pub user_id: Uuid,
}
