use axum::body::Bytes;
use object_store::{aws::AmazonS3Builder, azure::MicrosoftAzureBuilder, ObjectStore};

use super::StorageBackend;

pub(crate) struct GeneralS3 {
    object_store: Box<dyn ObjectStore>,
}

impl GeneralS3 {
    pub(crate) fn new_minio(
        bucket_url: &str,
        access_key_id: &str,
        secret_access_key: &str,
        bucket: &str,
    ) -> Self {
        let mut s3_builder = AmazonS3Builder::new()
            .with_access_key_id(access_key_id)
            .with_secret_access_key(secret_access_key)
            .with_endpoint(bucket_url)
            .with_bucket_name(bucket)
            .with_virtual_hosted_style_request(false); // MinIO requires path-style access

        if bucket_url.starts_with("http://") {
            s3_builder = s3_builder.with_allow_http(true);
        }

        let s3 = s3_builder.build().expect("Error creating S3 client");

        Self {
            object_store: Box::new(s3),
        }
    }

    pub(crate) fn new_azure(account: &str, access_key: &str, container: &str) -> Self {
        let azure_builder = MicrosoftAzureBuilder::new()
            .with_account(account)
            .with_access_key(access_key)
            .with_container_name(container);

        let azure = azure_builder
            .build()
            .expect("Error creating Azure Blob client");

        Self {
            object_store: Box::new(azure),
        }
    }

    pub(crate) fn new_hetzner(
        access_key: &str,
        secret_key: &str,
        bucket_url: &str,
        bucket_name: &str,
        region: &str,
    ) -> Self {
        let s3_builder = AmazonS3Builder::new()
            .with_access_key_id(access_key)
            .with_secret_access_key(secret_key)
            .with_region(region)
            .with_endpoint(bucket_url)
            .with_bucket_name(bucket_name);

        let s3 = s3_builder.build().expect("Error creating S3 client");

        Self {
            object_store: Box::new(s3),
        }
    }
}

#[async_trait::async_trait]
impl StorageBackend for GeneralS3 {
    async fn delete_file(
        &self,
        file_name: &str,
    ) -> Result<(), crate::storage::errors::StorageError> {
        let file_path = object_store::path::Path::from(file_name);
        self.object_store.delete(&file_path).await.map_err(|e| {
            tracing::error!("Failed to delete file: {:?}", e);
            crate::storage::errors::StorageError::Delete
        })
    }

    async fn store_file(
        &self,
        bytes: Vec<u8>,
        target_file_name: &str,
    ) -> Result<(), crate::storage::errors::StorageError> {
        let file_path = object_store::path::Path::from(target_file_name);

        let bytes = Bytes::from_owner(bytes);
        let put_payload = object_store::PutPayload::from_bytes(bytes);

        self.object_store
            .put(&file_path, put_payload)
            .await
            .map_err(|e| {
                tracing::error!("Failed to store file: {:?}", e);
                crate::storage::errors::StorageError::Write
            })?;
        Ok(())
    }

    async fn extract_file_bytes(
        &self,
        file_name: &str,
    ) -> Result<Vec<u8>, crate::storage::errors::StorageError> {
        let file_path = object_store::path::Path::from(file_name);
        let result = self.object_store.get(&file_path).await.map_err(|e| {
            tracing::error!("Failed to extract file bytes: {:?}", e);
            crate::storage::errors::StorageError::Read
        })?;
        let bytes = result.bytes().await.map_err(|e| {
            tracing::error!("Failed to extract file bytes: {:?}", e);
            crate::storage::errors::StorageError::Read
        })?;
        Ok(bytes.to_vec())
    }
}
