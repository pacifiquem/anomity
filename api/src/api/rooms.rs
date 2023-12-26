use sqlx::PgPool;

use crate::models::Room;

impl Room {
    pub async fn create_room(creator_id: i32, name: Option<String>, pool: &PgPool) {
        let rows = sqlx::query!(
            r#"INSERT INTO rooms(name, user_id) VALUES($1, $2) RETURNING id"#,
            name.unwrap(),
            creator_id
        )
        .fetch_one(pool)
        .await
        .unwrap();

        println!("{:?}", rows);
    }

    pub async fn delete_room(id: i32, user_id: i32, pool: &PgPool) {
        let result = sqlx::query!(
            r#"DELETE FROM rooms WHERE id=$1 AND user_id=$2"#,
            id,
            user_id
        )
        .execute(pool)
        .await
        .unwrap();

        println!("{:?}", result);
    }

    pub async fn get_rooms(user_id: i32, pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(Self, r#"SELECT * FROM rooms WHERE user_id=$1"#, user_id)
            .fetch_all(pool)
            .await
            .unwrap()
    }
}
