use std::net::SocketAddr;

use anyhow::Result;
use sea_orm::ConnectionTrait;
use tokio::signal;
use tracing::info;

mod config;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use config::AppConfig;
use routes::create_router;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file (if present)
    dotenvy::dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,restrosync=debug".into()),
        )
        .with_target(true)
        .with_thread_ids(false)
        .init();

    let config = AppConfig::from_env()?;
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    // Connect to PostgreSQL
    info!("Connecting to database...");
    let db = sea_orm::Database::connect(&config.database_url).await?;

    // Verify database connectivity
    db.execute_unprepared("SELECT 1").await.map_err(|e| {
        tracing::error!("Database connectivity check failed: {}", e);
        e
    })?;
    info!("Database connected and verified");

    let app = create_router();

    info!("RestroSync server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Drop database connection on shutdown
    drop(db);
    info!("Server shut down gracefully");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, starting graceful shutdown");
}
