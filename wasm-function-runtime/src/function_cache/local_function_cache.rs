pub(crate) struct LocalFunctionCache {
    inner_cache: moka::future::Cache<String, Vec<u8>>,
}

impl Default for LocalFunctionCache {
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
impl super::FunctionCacheBackend for LocalFunctionCache {
    async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.inner_cache.get(key).await
    }

    async fn insert(&self, key: &str, value: Vec<u8>) {
        self.inner_cache.insert(key.to_owned(), value).await;
    }

    async fn invalidate(&self, key: &str) {
        self.inner_cache.remove(key).await;
    }
}
