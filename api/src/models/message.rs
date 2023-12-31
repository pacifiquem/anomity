use async_session::chrono::FixedOffset;
use sqlx::types::chrono::DateTime as SqlxDateTime;
use uuid::Uuid;

pub struct _Message {
    id: i32,
    room_id: i32,
    user_id: Uuid,
    message: String,
    created_at: SqlxDateTime<FixedOffset>,
}
