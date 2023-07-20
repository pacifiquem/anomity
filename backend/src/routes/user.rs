use crate::{
    api::{generate_token, hash, login, KEYS},
    models::{Claims, SignUpRequest, User},
    AppState,
};
use async_session::async_trait;
use axum::{
    extract::{Extension, FromRequestParts, Path, State},
    http::request::Parts,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, Validation};
use sqlx::PgPool;

use uuid::Uuid;
use validator::Validate;

use crate::error::Error;
use crate::Result;

pub fn routes<S>(state: AppState) -> Router<S> {
    //Router::new().route("/", post(sign_up).get(get_all_users))
    Router::new()
        .route("/", get(get_all_users))
        .route("/:id", get(get_user))
        .with_state(state)
    //.route("/login", post(login))
    //.route("/me", get(get_current_user))
}

//async fn sign_up(, Json(req): Json<SignUpRequest>) -> impl IntoResponse {
//    req.validate()?;

//    let user = sqlx::query_as!(
//        User,
//        r#"
//		SELECT * FROM "users" WHERE email = $1
//	"#,
//        req.email
//    )
//    .fetch_one(&*db)
//    .await
//    .ok();

//    if let Some(_) = user {
//        return Err(Error::Conflict("User already exists".to_string()));
//    }

//    let password_hash = hash(req.password).await?;

//    sqlx::query_as!(
//        User,
//        r#"
//        INSERT INTO "users" (email,username,password)
//        VALUES ($1, $2, $3)
//    "#,
//        req.email,
//        req.username,
//        password_hash
//    )
//    .execute(&*db)
//    .await?;

//    let token = generate_token(req.email);

//    Ok(token)
//}

//async fn get_current_user(db: Extension<PgPool>, claims: Claims) -> Result<Json<User>> {
//    let user = sqlx::query_as!(
//        User,
//        r#"
//		SELECT * FROM "users" WHERE email = $1
//	"#,
//        claims.sub
//    )
//    .fetch_one(&*db)
//    .await
//    .map_err(|e| match e {
//        sqlx::Error::RowNotFound => Error::NotFound("Invalid credentials".to_string()),
//        _ => Error::Sqlx(e),
//    })?;

//    Ok(Json(user))
//}

async fn get_all_users(state: State<AppState>) -> Result<Json<Vec<User>>> {
    let users = User::get_all_users(&state.pg_pool).await;
    Ok(Json(users))
}

async fn get_user(state: State<AppState>, Path(user_id): Path<Uuid>) -> Result<Json<User>> {
    if let Some(user) = User::find_by_id(user_id, &state.pg_pool).await {
        return Ok(Json(user));
    }
    return Err(Error::NotFound("User not found".to_string()));
}

//#[async_trait]
//impl<S> FromRequestParts<S> for Claims
//where
//    S: Send + Sync,
//{
//    type Rejection = Error;

//    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//        let bearer = "Bearer ";

//        let authorization_header = match req.headers.get("authorization") {
//            Some(v) => v,
//            None => return Err(Error::Unauthorized("Invalid token".to_string())),
//        };

//        let authorization = match authorization_header.to_str() {
//            Ok(v) => v,
//            Err(_) => return Err(Error::Unauthorized("Invalid token".to_string())),
//        };

//        if !authorization.starts_with(bearer) {
//            return Err(Error::Unauthorized("Invalid token".to_string()));
//        };

//        let token_data = decode::<Claims>(
//            authorization.trim_start_matches(bearer),
//            &KEYS.decoding,
//            &Validation::default(),
//        )
//        .map_err(|_| Error::InvalidToken("Invalid token".to_string()))?;

//        Ok(token_data.claims)
//    }
//}
