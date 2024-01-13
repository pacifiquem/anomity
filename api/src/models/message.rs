use async_session::chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime as SqlxDateTime;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub message: String,
    pub created_at: SqlxDateTime<FixedOffset>,
}
