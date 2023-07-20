use anyhow::{anyhow, Context};
use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{Extension, Json};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use tokio::task;
use validator::Validate;

use crate::{
    error::Error,
    models::{Claims, SignInRequest, User},
};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secrets: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secrets),
            decoding: DecodingKey::from_secret(secrets),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub async fn login(db: Extension<PgPool>, Json(req): Json<SignInRequest>) -> Result<String, Error> {
    req.validate()?;

    let user = sqlx::query_as!(
        User,
        r#"
		SELECT * FROM users WHERE email = $1
	"#,
        req.email
    )
    .fetch_one(&*db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound("Invalid credentials".to_string()),
        _ => Error::Sqlx(e),
    })?;

    let is_valid = verify(req.password, user.password).await?;

    if !is_valid {
        return Err(Error::Unauthorized("Invalid credentials".to_string()));
    }

    let token = generate_token(user.email);

    Ok(token)
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

pub fn generate_token(email: String) -> String {
    let claims = Claims {
        sub: email,
        exp: (time::OffsetDateTime::now_utc() + time::Duration::weeks(1)).unix_timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| Error::TokenCreation("Failed to create token".to_string()))
        .unwrap();

    token
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
    .context("Failed to hash")?
}