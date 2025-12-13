use anyhow::Result;
use redis::aio::ConnectionManager;
use redis::Client;
use tracing::info;

use crate::config::RedisConfig;

pub async fn create_pool(config: &RedisConfig) -> Result<ConnectionManager> {
    let client = Client::open(config.url.as_str())?;
    let manager = ConnectionManager::new(client).await?;

    info!("Redis connection manager created");

    Ok(manager)
}
