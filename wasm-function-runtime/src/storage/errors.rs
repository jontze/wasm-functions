use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::utils::ErrorResponse;

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

impl IntoResponse for StorageError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error",
            }),
        )
            .into_response()
    }
}
