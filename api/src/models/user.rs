use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::PgPool;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

#[serde_with::serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,

    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,

    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl User {
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Option<Self> {
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

    //pub async fn create_user(
    //    username: &str,
    //    email: &str,
    //    password: &str,
    //    pool: &PgPool,
    //) -> Option<Self> {
    //}
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignInRequest {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 32,
        message = "Password must be between 6 and 32 characters long"
    ))]
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUpRequest {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 32,
        message = "Username must be between 3 and 32 characters long"
    ))]
    pub username: String,

    #[validate(length(
        min = 6,
        max = 32,
        message = "Password must be between 6 and 32 characters long"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
