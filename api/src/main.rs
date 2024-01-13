use std::{collections::HashSet, net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{routing::get, Router};
use futures::lock::Mutex;
use sqlx::{PgPool, Pool, Postgres};
use tokio::signal::unix::SignalKind;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod error;
mod models;
mod routes;

use crate::db::connect_pg;
use tokio::sync::broadcast;

use self::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

struct AppState {
    pg_pool: PgPool,
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
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
        .context("Failed to connect to database")
        .unwrap();

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .context("Failed to run migrations")?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));

    tracing::debug!("listening on {}", addr);

    //let cors = CorsLayer::new().allow_methods([]).allow_origin(Any);
    //.layer(cors);

    axum::Server::bind(&addr)
        .serve(app(pg_pool).await.into_make_service())
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

async fn app(pg_pool: Pool<Postgres>) -> Router {
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        pg_pool,
        user_set,
        tx,
    });

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/api", routes::all_routes(app_state))
}

#[cfg(test)]
mod tests;
