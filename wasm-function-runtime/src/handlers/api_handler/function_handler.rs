use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    domain::{self, function::WasmFunctionTrait},
    function_service, RuntimeStateRef,
};

pub(super) fn router() -> Router<RuntimeStateRef> {
    Router::new()
        .route("/", get(list_scope_functions))
        .route("/http/{function_id}", delete(delete_http_function))
        .route(
            "/scheduled/{function_id}",
            delete(delete_scheduled_function),
        )
}

#[derive(Deserialize)]
struct FunctionPath {
    #[allow(unused)]
    scope: String,
    function_id: Uuid,
}

#[derive(Serialize)]
struct ScopeFunctionItem {
    name: String,
    uuid: Uuid,
    kind: String,
}

#[derive(Serialize)]
struct ScopeFunctionListResponse {
    functions: Vec<ScopeFunctionItem>,
}

impl From<Vec<domain::function::Function>> for ScopeFunctionListResponse {
    fn from(functions: Vec<domain::function::Function>) -> Self {
        Self {
            functions: functions
                .into_iter()
                .map(|func| ScopeFunctionItem {
                    name: func.name().to_owned(),
                    uuid: func.uuid(),
                    kind: func.kind().to_owned(),
                })
                .collect(),
        }
    }
}

async fn list_scope_functions(
    State(state): State<RuntimeStateRef>,
    Path(scope_name): Path<String>,
) -> impl IntoResponse {
    function_service::find_all_funcs(&state.db, &scope_name)
        .await
        .map(ScopeFunctionListResponse::from)
        .map(Json)
        .into_response()
}

async fn delete_http_function(
    State(state): State<RuntimeStateRef>,
    Path(path): Path<FunctionPath>,
) -> impl IntoResponse {
    function_service::delete_http_func(
        &state.db,
        &*state.cache_backend,
        &*state.storage_backend,
        &path.function_id,
    )
    .await
    .map(|_| StatusCode::ACCEPTED)
    .into_response()
}

async fn delete_scheduled_function(
    State(state): State<RuntimeStateRef>,
    Path(path): Path<FunctionPath>,
) -> impl IntoResponse {
    function_service::delete_scheduled_func(
        &state.db,
        &*state.scheduler_manager,
        &*state.storage_backend,
        &path.function_id,
    )
    .await
    .map(|_| StatusCode::ACCEPTED)
    .into_response()
}
