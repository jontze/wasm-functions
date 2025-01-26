use super::StorageBackend;

pub(crate) struct CachedStorage {
    storage_backend: Box<dyn StorageBackend>,
    local_cache: moka::future::Cache<String, Vec<u8>>,
}

impl CachedStorage {
    pub(crate) fn new(storage_backend: Box<dyn StorageBackend>) -> Self {
        Self {
            storage_backend,
            local_cache: moka::future::Cache::builder()
                .time_to_idle(
                    std::time::Duration::from_secs(60 * 60 * 24), /* 24 hours after last access */
                )
                .build(),
        }
    }
}

impl Default for CachedStorage {
    /// By default, use the local file system storage backend and cache it
    fn default() -> Self {
        Self::new(Box::new(
            crate::storage::file_system::FileSystemStorage::new(),
        ))
    }
}

#[async_trait::async_trait]
impl StorageBackend for CachedStorage {
    async fn store_file(
        &self,
        bytes: Vec<u8>,
        target_file_name: &str,
    ) -> Result<(), crate::storage::errors::StorageError> {
        self.storage_backend
            .store_file(bytes, target_file_name)
            .await
    }

    async fn extract_file_bytes(
        &self,
        file_name: &str,
    ) -> Result<Vec<u8>, crate::storage::errors::StorageError> {
        if let Some(bytes) = self.local_cache.get(file_name).await {
            Ok(bytes)
        } else {
            let bytes = self.storage_backend.extract_file_bytes(file_name).await?;
            self.local_cache
                .insert(file_name.to_string(), bytes.to_owned())
                .await;
            Ok(bytes)
        }
    }

    async fn delete_file(
        &self,
        file_name: &str,
    ) -> Result<(), crate::storage::errors::StorageError> {
        self.local_cache.invalidate(file_name).await;
        self.storage_backend.delete_file(file_name).await
    }
}
