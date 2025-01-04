use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum StorageError {
    #[error("Unable to create storage directory")]
    CreateStorageDirectory,
    #[error("Unable to write file")]
    Write,
    #[error("Unable to read file")]
    Read,
    #[error("Unable to delete file")]
    Delete,
}
