use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::utils::ErrorResponse;

#[derive(Debug, Error)]
pub(crate) enum ServiceError {
    #[error("Interaction with Database failed")]
    Db(#[from] sea_orm::DbErr),
    #[error("Interaction with Storage Backend failed")]
    Storage(#[from] crate::storage::errors::StorageError),
    #[error("Interaction with Cache Backend failed")]
    Cache(#[from] crate::cache::error::CacheError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            ServiceError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error",
                }),
            )
                .into_response(),
            ServiceError::Storage(storage_err) => storage_err.into_response(),
            ServiceError::Cache(cache_err) => cache_err.into_response(),
        }
    }
}
