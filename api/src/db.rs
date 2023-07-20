use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect_pg() -> anyhow::Result<PgPool> {
    let db_connection = dotenvy::var("DATABASE_URL").context("Database URL not set.")?;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection)
        .await
        .context("Failed to connect to Postgres.")
}
