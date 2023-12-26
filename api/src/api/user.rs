use sqlx::PgPool;

use crate::models::User;

impl User {
    pub async fn get_by_id(id: i32, pool: &PgPool) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE id=$1", id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    pub async fn get_all_users(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT * FROM users")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn create(email: &str, username: &str, password: &str, pool: &PgPool) -> i32 {
        let result = sqlx::query!(
            r#"
            INSERT INTO "users" (email,username,password)
            VALUES ($1, $2, $3) 
            RETURNING id
        "#,
            email,
            username,
            password
        )
        .fetch_one(pool)
        .await
        .unwrap();

        result.id
    }

    pub async fn get_by_email(email: &str, pool: &PgPool) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE email=$1", email)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}
