use axum::Extension;
use axum::{routing::post, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::PgPool;
use uuid::Uuid;

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use validator::Validate;

use crate::Result;

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: Uuid,
    username: String,
    email: String,
    password: String,

    #[serde_as(as = "Rfc3339")]
    created_at: OffsetDateTime,

    #[serde_as(as = "Rfc3339")]
    updated_at: OffsetDateTime,
}

pub fn routes() -> Router {
    Router::new().route("/api/users", post(create_user).get(get_all_users))
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
struct UserCreateRequest {
    email: String,
    username: String,

    #[validate(length(min = 8, max = 32))]
    password: String,
}

async fn create_user(
    db: Extension<PgPool>,
    Json(req): Json<UserCreateRequest>,
) -> Result<StatusCode> {
    req.validate()?;

    // TODO: check if user already exists

    print!("{:?}", req);
    sqlx::query_as!(
        User,
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

    Ok(StatusCode::NO_CONTENT)
}

async fn get_all_users(db: Extension<PgPool>) -> Result<Json<Vec<User>>> {
    let users = sqlx::query_as!(
        User,
        r#"
		SELECT * FROM "users"
	"#
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(users))
}

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
