use anyhow::Result;
use sea_orm_migration::MigratorTrait;

use restrosync_server::migrations::Migrator;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://restrosync:restrosync_dev@localhost:5432/restrosync".into()
    });

    tracing::info!("Running migrations against: {}", database_url);

    let db = sea_orm::Database::connect(&database_url).await?;

    Migrator::up(&db, None).await?;

    tracing::info!("Migrations completed successfully");
    drop(db);

    Ok(())
}
