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
    DbInteraction(#[from] sea_orm::DbErr),
    #[error("Interaction with Storage Backend failed")]
    StorageInteraction(#[from] crate::storage::errors::StorageError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        match self {
            ServiceError::DbInteraction(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error",
                }),
            )
                .into_response(),
            ServiceError::StorageInteraction(storage_err) => storage_err.into_response(),
        }
    }
}
