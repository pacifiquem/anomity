use std::sync::Arc;

use crate::{
    api::{generate_token, hash, KEYS},
    models::{Claims, SignUpRequest, User},
    AppState,
};
use async_session::async_trait;
use axum::{
    extract::{FromRequestParts, Path, State},
    response::IntoResponse,
    Json,
};

use hyper::http::request::Parts;
use jsonwebtoken::{decode, Validation};
use validator::Validate;

use crate::error::Error;
use crate::Result;

pub async fn create(
    state: State<Arc<AppState>>,
    Json(req): Json<SignUpRequest>,
) -> impl IntoResponse {
    req.validate()?;

    let user = User::get_by_email(&req.email, &state.pg_pool).await;

    if user.is_some() {
        return Err(Error::Conflict("User already exists".to_string()));
    }

    let password_hash = hash(req.password).await?;

    let id = User::create(&req.email, &req.username, &password_hash, &state.pg_pool).await;

    return Ok(generate_token(id));
}

pub async fn get_current_user(state: State<Arc<AppState>>, claims: Claims) -> Result<Json<User>> {
    let user = User::get_by_id(claims.sub, &state.pg_pool).await;

    if let Some(user) = user {
        return Ok(Json(user));
    }

    Err(Error::NotFound("User not found".to_owned()))
}

pub async fn get_all_users(state: State<Arc<AppState>>) -> Result<Json<Vec<User>>> {
    let users = User::get_all_users(&state.pg_pool).await;
    Ok(Json(users))
}

pub async fn get_user(state: State<Arc<AppState>>, Path(user_id): Path<i32>) -> Result<Json<User>> {
    if let Some(user) = User::get_by_id(user_id, &state.pg_pool).await {
        return Ok(Json(user));
    }
    Err(Error::NotFound("User not found".to_string()))
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = "Bearer ";

        let authorization_header = match req.headers.get("authorization") {
            Some(v) => v,
            None => return Err(Error::Unauthorized("Invalid token".to_string())),
        };

        let authorization = match authorization_header.to_str() {
            Ok(v) => v,
            Err(_) => return Err(Error::Unauthorized("Invalid token".to_string())),
        };

        if !authorization.starts_with(bearer) {
            return Err(Error::Unauthorized("Invalid token".to_string()));
        };

        let token_data = decode::<Claims>(
            authorization.trim_start_matches(bearer),
            &KEYS.decoding,
            &Validation::default(),
        )
        .map_err(|_| Error::InvalidToken("Invalid token".to_string()))?;

        Ok(token_data.claims)
    }
}
