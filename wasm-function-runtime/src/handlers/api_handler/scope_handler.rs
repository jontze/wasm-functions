use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Json, Router,
};
use serde::Serialize;

use crate::{domain, server_state::RuntimeStateRef, services::scope_service};

pub(super) fn router() -> Router<RuntimeStateRef> {
    Router::new()
        .route("/", get(list_scopes))
        .route("/{scope}", delete(delete_scope))
}

#[derive(Serialize)]
struct ScopeListResponse {
    scopes: Vec<domain::scope::FunctionScope>,
}

impl From<Vec<domain::scope::FunctionScope>> for ScopeListResponse {
    fn from(scopes: Vec<domain::scope::FunctionScope>) -> Self {
        Self { scopes }
    }
}

async fn list_scopes(State(state): State<RuntimeStateRef>) -> impl IntoResponse {
    scope_service::get_all_scopes(&state.db)
        .await
        .map(ScopeListResponse::from)
        .map(Json)
        .into_response()
}

async fn delete_scope(
    State(state): State<RuntimeStateRef>,
    Path(scope_name): Path<String>,
) -> impl IntoResponse {
    scope_service::delete_scope(&state.db, &scope_name)
        .await
        .map(|_| StatusCode::ACCEPTED)
        .into_response()
}
