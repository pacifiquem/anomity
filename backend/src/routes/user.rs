use anyhow::{anyhow, Context};
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::{Path, FromRequest};
use axum::http::request::Parts;
use axum::routing::get;
use axum::{Extension, async_trait, RequestPartsExt};
use axum::{routing::post, Json, Router};
use hyper::StatusCode;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::PgPool;
use tokio::task;
use uuid::Uuid;

use argon2::password_hash::SaltString;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use validator::Validate;

use crate::error::Error;
use crate::Result;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secrets: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secrets),
            decoding: DecodingKey::from_secret(secrets),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

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
    Router::new()
        .route("/api/users", post(sign_up).get(get_all_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/users/signin", post(sign_in))
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
struct SignUpRequest {
    email: String,
    username: String,

    #[validate(length(min = 6, max = 32))]
    password: String,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
struct SingnInRequest {
    email: String,
    password: String,
}

#[serde_with::serde_as]
#[derive(Deserialize, Serialize,Debug)]
#[serde(rename_all = "camelCase")]
struct SignInResponse {
    token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn sign_up(db: Extension<PgPool>, Json(req): Json<SignUpRequest>) -> Result<StatusCode> {
    req.validate()?;

    let user = sqlx::query_as!(
        User,
        r#"
		SELECT * FROM "users" WHERE email = $1
	"#,
        req.email
    )
    .fetch_one(&*db)
    .await
    .ok();

    if let Some(_) = user {
        return Err(Error::Conflict("User already exists".to_string()));
    }

    let password_hash = hash(req.password).await?;

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO "users" (email,username,password)
        VALUES ($1, $2, $3)
    "#,
        req.email,
        req.username,
        password_hash
    )
    .execute(&*db)
    .await?;

    Ok(StatusCode::CREATED)
}

async fn sign_in(
    db: Extension<PgPool>,
    Json(req): Json<SingnInRequest>,
) -> Result<Json<SignInResponse>> {
    req.validate()?;

    let user = sqlx::query_as!(
        User,
        r#"
		SELECT * FROM "users" WHERE email = $1
	"#,
        req.email
    )
    .fetch_one(&*db)
    .await?;

    let is_valid = verify(req.password, user.password).await?;

    if !is_valid {
        return Err(Error::Unauthorized("Invalid credentials".to_string()));
    }

    let claims = Claims {
        sub: user.email,
        exp: 100000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| Error::TokenCreation("Failed to create token".to_string()))?;

    Ok(Json(SignInResponse { token }))
}

async fn get_all_users(db: Extension<PgPool>) -> Result<Json<Vec<User>>> {
    let users = sqlx::query_as!(
        User,
        r#"
		SELECT * FROM users
	    "#
    )
    .fetch_all(&*db)
    .await?;
    Ok(Json(users))
}

async fn get_user(db: Extension<PgPool>, Path(user_id): Path<Uuid>) -> Result<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
	SELECT * FROM "users" WHERE id = $1
	"#,
        user_id
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound("User not found".to_string()),
        _ => Error::Sqlx(e),
    })?;

    Ok(Json(user))
}

pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());

        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("Failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash()")?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash =
            PasswordHash::new(&hash).map_err(|e| anyhow!(e).context("Failed to parse hash"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("Failed to verify password")),
        }
    })
    .await
    .context("panic in verify()")?
}


#[async_trait]
impl<S> FromRequest<S> for Claims where S: Send + Sync {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		let TypedHeader(Authorization(bearer)) = parts
		.extract::<TypedHeader<Authorization<Bearer>>>()
		.await
		.map_err(|_| Error::Unauthorized("Missing authorization header".to_string()))?;

	  let token_data = decode::<Claims>(&bearer.token, &KEYS.decoding, &Validation::default())
	  .map_err(|_| Error::InvalidToken("Invalid token".to_string()))?;

	 Ok(token_data.claims)
	}
}