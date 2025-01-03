use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain,
    server_state::RuntimeStateRef,
    services::{function_service, scope_service},
};

pub(crate) fn router(
    app_state: crate::server_state::RuntimeStateRef,
) -> axum::routing::Router<RuntimeStateRef> {
    axum::Router::new()
        .route("/deploy", post(deploy_function_with_manifest))
        .route("/scope", get(list_scopes))
        .route("/scope/:scope", delete(delete_scope))
        .route("/scope/:scope/function", get(list_scope_functions))
        .route(
            "/scope/:scope/function/http/:function_id",
            delete(delete_http_function),
        )
        .route(
            "/scope/:scope/function/scheduled/:function_id",
            delete(delete_scheduled_function),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state,
            crate::middlewares::auth::auth,
        ))
}

#[derive(Default)]
pub(crate) struct CreateHttpFunctionPayload {
    pub name: String,
    pub method: String,
    pub scope: String,
    pub path: String,
    pub is_public: bool,
    pub wasm_bytes: Vec<u8>,
}

#[derive(Default)]
#[allow(unused)]
pub(crate) struct CreateScheduledFunctionPayload {
    pub name: String,
    pub scope: String,
    pub cron: String,
    pub wasm_bytes: Vec<u8>,
}

async fn deploy_function_with_manifest(
    State(state): State<RuntimeStateRef>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let mut manifest: Option<domain::manifest::Manifest> = None;
    let mut wasm_bytes: Vec<u8> = vec![];

    while let Some(field) = multipart.next_field().await.expect("Failed to read file") {
        match field.file_name().expect("Failed to get file name") {
            "manifest.toml" => {
                let data = field.bytes().await.expect("Failed to read field");
                manifest = Some(
                    toml::from_str::<domain::manifest::Manifest>(
                        std::str::from_utf8(&data).expect("Failed to parse manifest"),
                    )
                    .expect("Failed to parse manifest"),
                );
            }
            file_name => {
                if file_name.ends_with(".wasm") {
                    wasm_bytes = field.bytes().await.expect("Failed to read field").to_vec();
                } else {
                    return Err(format!("Invalid file name: {}", file_name).into_response());
                }
            }
        };
    }

    if wasm_bytes.is_empty() {
        return Err("Wasm file is required".into_response());
    }

    // Push manifest data to the corresponding payload
    if let Some(manifest) = manifest {
        match manifest.function.trigger {
            domain::manifest::FuncKind::Http => {
                if let Some(http) = &manifest.http {
                    let payload = CreateHttpFunctionPayload {
                        name: manifest.function.name,
                        scope: manifest.function.scope,
                        method: http.method.as_ref().to_string(),
                        path: http.path.clone(),
                        is_public: http.public,
                        wasm_bytes,
                    };

                    function_service::create_http_func(&state.db, &state.registry, payload).await;
                } else {
                    return Err("HTTP function must have HTTP section in manifest".into_response());
                }
            }
            domain::manifest::FuncKind::Scheduled => {
                if let Some(scheduled) = &manifest.scheduled {
                    let payload = CreateScheduledFunctionPayload {
                        name: manifest.function.name,
                        scope: manifest.function.scope,
                        cron: scheduled.cron.clone(),
                        wasm_bytes,
                    };

                    function_service::create_scheduled_func(
                        &state.db,
                        &*state.scheduler_manager,
                        payload,
                    )
                    .await;
                } else {
                    return Err("Scheduled function must have scheduled section".into_response());
                }
            }
        };
    } else {
        return Err("Manifest file is required".into_response());
    }

    Ok("OK".into_response())
}

async fn delete_scope(
    State(state): State<RuntimeStateRef>,
    Path(scope_name): Path<String>,
) -> impl IntoResponse {
    scope_service::delete_scope(&state.db, &scope_name).await;

    "OK".into_response()
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
    let scopes: ScopeListResponse = scope_service::get_all_scopes(&state.db).await.into();
    Json(scopes).into_response()
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
    let functions: ScopeFunctionListResponse =
        function_service::find_all_funcs(&state.db, &scope_name)
            .await
            .into();
    Json(functions).into_response()
}

#[derive(Deserialize)]
struct FunctionPath {
    #[allow(unused)]
    scope: String,
    function_id: Uuid,
}

async fn delete_http_function(
    State(state): State<RuntimeStateRef>,
    Path(path): Path<FunctionPath>,
) -> impl IntoResponse {
    function_service::delete_http_func(&state.db, &state.registry, &path.function_id).await;

    "OK".into_response()
}

async fn delete_scheduled_function(
    State(state): State<RuntimeStateRef>,
    Path(path): Path<FunctionPath>,
) -> impl IntoResponse {
    function_service::delete_scheduled_func(
        &state.db,
        &*state.scheduler_manager,
        &path.function_id,
    )
    .await;

    "OK".into_response()
}
