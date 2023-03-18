use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Serialize;
use serde_with::serde_as;
use sqlx::PgPool;
use uuid::Uuid;

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
#[derive(Serialize)]
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

pub struct UserCreateRequest {
    email: String,
    username: String,
    password: String,
}

async fn create_user(
    db: Extension<PgPool>,
    Json(req): Json<UserCreateRequest>,
) -> anyhow::Result<StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO "users" (email,username,password)
        VALUES ($1, $2, $3)
    "#,
        req.email,
        req.username,
        req.password
    )
    .execute(&*db)
    .await
    .map_err(|e| {
        println!("Error: {}", e);
    });

    Ok(StatusCode::CREATED)
}

async fn get_user(db: Extension<PgPool>, Path(user_id): Path<Uuid>) -> anyhow::Result<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
	SELECT * FROM "users" WHERE id = $1
	"#,
        user_id
    )
    .fetch_one(&*db)
    .await?;

    Ok(Json(user))
}
