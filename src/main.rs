mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use config::Config;
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_backend_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting Rust Backend Starter...");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("✅ Configuration loaded");

    // Create database connection pool
    let db_pool = db::create_pool(&config.database_url).await?;

    // Run migrations
    db::run_migrations(&db_pool).await?;

    // Create application state
    let state = AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Create router
    let app = routes::create_router(state, config.clone());

    // Start server
    let addr: SocketAddr = config.server_address().parse()?;
    tracing::info!("🌐 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
