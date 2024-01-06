use async_session::chrono::FixedOffset;
use sqlx::types::chrono::DateTime as SqlxDateTime;

pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub message: String,
    pub created_at: SqlxDateTime<FixedOffset>,
}
