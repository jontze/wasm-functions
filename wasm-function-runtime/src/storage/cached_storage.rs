use std::sync::Arc;

use crate::cache::CacheBackend;

use super::StorageBackend;

pub(crate) struct CachedStorage {
    storage_backend: Box<dyn StorageBackend>,
    cache_backend: Arc<dyn CacheBackend>,
}

impl CachedStorage {
    pub(crate) fn new(
        storage_backend: Box<dyn StorageBackend>,
        cache_backend: Arc<dyn CacheBackend>,
    ) -> Self {
        Self {
            storage_backend,
            cache_backend,
        }
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
        if let Some(bytes) = self.cache_backend.get(file_name).await? {
            Ok(bytes)
        } else {
            let bytes = self.storage_backend.extract_file_bytes(file_name).await?;
            self.cache_backend
                .insert(file_name, bytes.to_owned())
                .await?;
            Ok(bytes)
        }
    }

    async fn delete_file(
        &self,
        file_name: &str,
    ) -> Result<(), crate::storage::errors::StorageError> {
        self.cache_backend.invalidate(file_name).await?;
        self.storage_backend.delete_file(file_name).await
    }
}
