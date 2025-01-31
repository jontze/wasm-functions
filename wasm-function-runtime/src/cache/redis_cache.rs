use redis::{aio::ConnectionManager, AsyncCommands};

use super::CacheError;

pub(crate) struct RedisCache {
    client: ConnectionManager,
}

impl RedisCache {
    pub(crate) async fn new(connection_str: &str) -> Self {
        let client = redis::Client::open(connection_str)
            .expect("Failed to create Redis client")
            .get_connection_manager()
            .await
            .expect("Failed to create Redis connection manager");
        Self { client }
    }
}

#[async_trait::async_trait]
impl super::CacheBackend for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        let mut con = self.client.clone();
        let cache_result: Option<Vec<u8>> = con.get(key).await.map_err(|e| {
            tracing::error!("Failed to get value from Redis: {:?}", e);
            CacheError::Read
        })?;

        Ok(cache_result)
    }

    async fn insert(&self, key: &str, value: Vec<u8>) -> Result<(), CacheError> {
        let mut con = self.client.clone();
        let _: () = con.set_ex(key, value, 60 * 60 * 24).await.map_err(|e| {
            tracing::error!("Failed to insert value into Redis: {:?}", e);
            CacheError::Write
        })?;
        Ok(())
    }

    async fn invalidate(&self, key: &str) -> Result<(), CacheError> {
        let mut con = self.client.clone();
        let _: () = con.del(key).await.map_err(|e| {
            tracing::error!("Failed to invalidate value in Redis: {:?}", e);
            CacheError::Delete
        })?;
        Ok(())
    }
}
