use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::utils::ErrorResponse;

#[derive(Debug, Error)]
pub(crate) enum CacheError {
    #[error("Unable to write to cache")]
    Write,
    #[error("Unable to read from cache")]
    Read,
    #[error("Unable to delete item from cache")]
    Delete,
}

impl IntoResponse for CacheError {
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
