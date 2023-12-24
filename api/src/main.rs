use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};
use sqlx::PgPool;
use tokio::signal::unix::SignalKind;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod error;
mod models;
mod routes;

use crate::db::connect_pg;

use self::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Clone)]
pub struct AppState {
    /// The secret to encrypt the JWT
    pub jwt_secret: String,

    /// The database connection pool
    pub pg_pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pg_pool = connect_pg()
        .await
        .context("Failed to connect to database")?;

    let jwt_secret = dotenvy::var("JWT_SECRET").context("JWT_SECRET not set")?;

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .context("Failed to run migrations")?;

    let state = AppState {
        jwt_secret,
        pg_pool,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));

    tracing::debug!("listening on {}", addr);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api/users", routes::all_routes(state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
            let mut sigkill = tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = sigterm.recv() => {},
                _ = sigkill.recv() => {},
            }
            tracing::info!("Received signal, starting graceful shutdown");
        })
        .await
        .context("Failed to start server")?;

    Ok(())
}
