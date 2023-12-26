use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Room;

impl Room {
    pub async fn create_room(creator_id: Uuid, name: Option<String>, pool: &PgPool) {
        let rows = sqlx::query("INSERT INTO rooms(name, user_id) VALUES($1, $2)")
            .bind(name)
            .bind(creator_id)
            .execute(pool)
            .await
            .unwrap();
    }
    pub async fn delete_room(_pool: &PgPool) {}

    // -> Vec<Self>
    pub async fn get_rooms(user_id: Uuid) {}
}
