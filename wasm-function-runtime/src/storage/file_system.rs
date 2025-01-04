use std::path::Path;
use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::error;

use super::{errors::StorageError, storage_backend::StorageBackend};

pub(crate) struct FileSystemStorage {
    function_dir: String,
}

impl FileSystemStorage {
    pub(crate) fn new() -> Self {
        Self {
            function_dir: "wasm_functions".to_string(),
        }
    }

    fn get_file_path(&self, file_name: &str) -> std::path::PathBuf {
        Path::new(self.function_dir.as_str()).join(file_name)
    }
}

impl Default for FileSystemStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl StorageBackend for FileSystemStorage {
    async fn store_file(&self, bytes: Vec<u8>, target_file_name: &str) -> Result<(), StorageError> {
        let file_path = self.get_file_path(target_file_name);

        // Create the folder if it doesn't exist
        if !file_path
            .parent()
            .ok_or(StorageError::CreateStorageDirectory)?
            .exists()
        {
            tokio::fs::create_dir_all(
                file_path
                    .parent()
                    .ok_or(StorageError::CreateStorageDirectory)?,
            )
            .await
            .inspect_err(|e| error!("Failed to create directory: {:?}", e))
            .map_err(|_| StorageError::CreateStorageDirectory)?;
        }

        // Create the file and write the bytes
        let mut file = TokioFile::create(file_path)
            .await
            .inspect_err(|e| error!("Failed to create file: {:?}", e))
            .map_err(|_| StorageError::Write)?;
        file.write_all(&bytes)
            .await
            .inspect_err(|e| error!("Failed to write file: {:?}", e))
            .map_err(|_| StorageError::Write)?;
        file.flush()
            .await
            .inspect_err(|e| error!("Failed to sync file: {:?}", e))
            .map_err(|_| StorageError::Write)?;
        Ok(())
    }

    async fn extract_file_bytes(&self, file_name: &str) -> Result<Vec<u8>, StorageError> {
        let file_path = self.get_file_path(file_name);

        let mut file = TokioFile::open(file_path)
            .await
            .inspect_err(|e| error!("Failed to open file: {:?}", e))
            .map_err(|_| StorageError::Read)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)
            .await
            .inspect_err(|e| error!("Failed to read file: {:?}", e))
            .map_err(|_| StorageError::Read)?;
        Ok(bytes)
    }

    async fn delete_file(&self, file_name: &str) -> Result<(), StorageError> {
        let file_path = self.get_file_path(file_name);

        tokio::fs::remove_file(file_path)
            .await
            .inspect_err(|e| error!("Failed to delete file: {:?}", e))
            .map_err(|_| StorageError::Delete)?;
        Ok(())
    }
}
