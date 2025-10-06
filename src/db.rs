use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::Context;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    tracing::info!("✅ Database connection established");
    
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
