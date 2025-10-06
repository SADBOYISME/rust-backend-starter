use anyhow::Context;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub app_env: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .context("PORT must be a valid number")?,
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET must be set")?,
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .context("JWT_EXPIRATION must be a valid number")?,
            app_env: env::var("APP_ENV")
                .unwrap_or_else(|_| "development".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
