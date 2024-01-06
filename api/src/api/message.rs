use sqlx::PgPool;

use crate::models::Message;

impl Message {
    pub async fn get_by_room_id(room_id: i32, pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT * FROM messages WHERE room_id=$1", room_id)
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn create(room_id: i32, user_id: i32, message: &str, pool: &PgPool) -> Self {
        let result = sqlx::query!(
            r#"
			INSERT INTO "messages" (room_id,user_id,message)
			VALUES ($1, $2, $3) 
			RETURNING *
		"#,
            room_id,
            user_id,
            message
        )
        .fetch_one(pool)
        .await
        .unwrap();

        Message {
            id: result.id,
            room_id: result.room_id,
            user_id: result.user_id,
            message: result.message,
            created_at: result.created_at.into(),
        }
    }
}
