use anyhow::{Context, Result};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3001".into())
                .parse()
                .context("SERVER_PORT must be a valid port number")?,
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://restrosync:restrosync_dev@localhost:5432/restrosync".into()
            }),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".into()),
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".into())
                .parse()
                .context("JWT_EXPIRY_HOURS must be a number")?,
        })
    }
}
