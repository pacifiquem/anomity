use async_session::chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime as SqlxDateTime;
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

    pub created_at: SqlxDateTime<FixedOffset>,

    pub updated_at: SqlxDateTime<FixedOffset>,
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
