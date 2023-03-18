use axum::http::StatusCode;
use axum::Extension;
use axum::{routing::post, Json, Router};
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

pub fn routes() -> Router {
    Router::new().route("/api/users", post(create_user))
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
    .await?;

    Ok(StatusCode::CREATED)
}

//async fn get_all_users(db: Extension<PgPool>) -> anyhow::Result<Json<Vec<User>>> {
//    let users = sqlx::query_as!(
//        User,
//        r#"
//		SELECT * FROM "users"
//	"#
//    )
//    .fetch_all(&*db)
//    .await?;

//    Ok(Json(users))
//}

//async fn get_user(db: Extension<PgPool>, Path(user_id): Path<Uuid>) -> anyhow::Result<Json<User>> {
//    let user = sqlx::query_as!(
//        User,
//		// language=PostgreSQL
//        r#"
//	SELECT * FROM "users" WHERE id = $1
//	"#,
//        user_id
//    )
//    .fetch_one(&*db)
//    .await?;

//    Ok(Json(user))
//}
