use super::CacheError;

pub(crate) struct LocalCache {
    inner_cache: moka::future::Cache<String, Vec<u8>>,
}

impl Default for LocalCache {
    fn default() -> Self {
        Self {
            inner_cache: moka::future::Cache::builder()
                .time_to_idle(std::time::Duration::from_secs(
                    60 * 60 * 24, /* 24 hours */
                ))
                .build(),
        }
    }
}

#[async_trait::async_trait]
impl super::CacheBackend for LocalCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        Ok(self.inner_cache.get(key).await)
    }

    async fn insert(&self, key: &str, value: Vec<u8>) -> Result<(), CacheError> {
        self.inner_cache.insert(key.to_owned(), value).await;
        Ok(())
    }

    async fn invalidate(&self, key: &str) -> Result<(), CacheError> {
        self.inner_cache.remove(key).await;
        Ok(())
    }
}
