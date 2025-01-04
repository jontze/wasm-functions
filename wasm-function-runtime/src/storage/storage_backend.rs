use super::errors::StorageError;

#[async_trait::async_trait]
pub(crate) trait StorageBackend: Send + Sync {
    async fn store_file(&self, bytes: Vec<u8>, target_file_name: &str) -> Result<(), StorageError>;
    async fn extract_file_bytes(&self, file_name: &str) -> Result<Vec<u8>, StorageError>;
    async fn delete_file(&self, file_name: &str) -> Result<(), StorageError>;
}
