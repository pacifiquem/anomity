use sqlx::PgPool;

use crate::models::Room;

impl Room {
    pub async fn create_room(creator_id: i32, name: Option<String>, pool: &PgPool) -> Room {
        let rows = sqlx::query!(
            r#"INSERT INTO rooms(name, user_id) VALUES($1, $2) RETURNING *"#,
            name.unwrap(),
            creator_id
        )
        .fetch_one(pool)
        .await
        .unwrap();

        sqlx::query!(
            r#"INSERT INTO rooms_participants(room_id,user_id) VALUES($1,$2)"#,
            rows.id,
            creator_id
        )
        .execute(pool)
        .await
        .unwrap();

        Room {
            user_id: rows.user_id,
            id: rows.id,
            name: rows.name,
            created_at: rows.created_at.into(),
        }
    }

    pub async fn delete_room(id: i32, user_id: i32, pool: &PgPool) -> Option<Room> {
        let result = sqlx::query!(
            r#"DELETE FROM rooms WHERE id=$1 AND user_id=$2 RETURNING *"#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await;

        if let Ok(room) = result {
            return Some(Room {
                id: room.id,
                user_id: room.user_id,
                name: room.name,
                created_at: room.created_at.into(),
            });
        }
        None
    }

    pub async fn get_rooms(user_id: i32, pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(Self, r#"SELECT * FROM rooms WHERE user_id=$1"#, user_id)
            .fetch_all(pool)
            .await
            .unwrap()
    }
}
