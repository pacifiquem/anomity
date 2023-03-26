use std::net::SocketAddr;

use anyhow::Context;
use axum::{Extension, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod error;
mod routes;

use self::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db_connection()
        .await
        .context("Failed to connect to database")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app(pool).into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
}

fn app(pool: PgPool) -> Router {
    Router::new()
        .merge(routes::user::routes())
        .layer(Extension(pool))
}

async fn db_connection() -> anyhow::Result<PgPool> {
    let db_connection = dotenvy::var("DATABASE_URL").context("Database URL not set.")?;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection)
        .await
        .context("Failed to connect to Postgres.")
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn root() {
        let app = app(db_connection().await.unwrap());

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, world!");
    }

    #[tokio::test]
    async fn not_found() {
        let app = app(db_connection().await.unwrap());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/not-found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
