#[async_trait::async_trait]
pub(crate) trait FunctionCacheBackend: Send + Sync {
    async fn get(&self, key: &str) -> Option<Vec<u8>>;
    async fn insert(&self, key: &str, value: Vec<u8>);
    async fn invalidate(&self, key: &str);
}
