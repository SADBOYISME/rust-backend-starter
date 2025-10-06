use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20) // Increased for better concurrency
        .min_connections(5) // Keep connections warm
        .acquire_timeout(Duration::from_secs(30)) // Prevent hangs
        .idle_timeout(Duration::from_secs(600)) // 10 min idle timeout
        .max_lifetime(Duration::from_secs(1800)) // 30 min max lifetime
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    tracing::info!("✅ Database pool created: max=20, min=5");

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Failed to run database migrations")?;

    tracing::info!("✅ Database migrations completed");

    Ok(())
}
