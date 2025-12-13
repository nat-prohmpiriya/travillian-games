use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::config::DatabaseConfig;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await?;

    // Test connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    info!("PostgreSQL connection pool created with {} max connections", config.max_connections);

    Ok(pool)
}
