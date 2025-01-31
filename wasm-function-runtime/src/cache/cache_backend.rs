use super::CacheError;

#[async_trait::async_trait]
pub(crate) trait CacheBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError>;
    async fn insert(&self, key: &str, value: Vec<u8>) -> Result<(), CacheError>;
    async fn invalidate(&self, key: &str) -> Result<(), CacheError>;
}
