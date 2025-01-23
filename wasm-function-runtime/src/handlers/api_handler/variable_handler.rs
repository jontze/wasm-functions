use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{domain, RuntimeStateRef};
use crate::services::variable_service;

pub(super) fn router() -> Router<RuntimeStateRef> {
    Router::new()
        .route("/", get(list_scope_variables))
        .route("/", post(create_scope_variable))
        .route("/{variable_id}", get(get_scope_variable))
        .route("/{variable_id}", put(update_scope_variable))
        .route("/{variable_id}", delete(delete_scope_variable))
}

#[derive(Serialize, Deserialize)]
struct ScopeVariablePath {
    scope: String,
    variable_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct CreateScopeVariablePayload {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateScopeVariablePayload {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Serialize)]
struct ScopeVariableResponse {
    uuid: String,
    name: String,
    value: String,
}

#[derive(Serialize)]
struct ScopeVariableListItem {
    uuid: String,
    name: String,
    value: String,
}

#[derive(Serialize)]
struct ScopeVariableListResponse {
    variables: Vec<ScopeVariableListItem>,
}

impl From<domain::variable::Variable> for ScopeVariableResponse {
    fn from(var: domain::variable::Variable) -> Self {
        Self {
            uuid: var.uuid.to_string(),
            name: var.name,
            value: var.value,
        }
    }
}

impl From<Vec<domain::variable::Variable>> for ScopeVariableListResponse {
    fn from(variables: Vec<domain::variable::Variable>) -> Self {
        Self {
            variables: variables
                .into_iter()
                .map(|var| ScopeVariableListItem {
                    uuid: var.uuid.to_string(),
                    name: var.name,
                    value: var.value,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct CreatedScopeVariableResponse {
    uuid: String,
    name: String,
    value: String,
}

impl From<domain::variable::Variable> for CreatedScopeVariableResponse {
    fn from(var: domain::variable::Variable) -> Self {
        Self {
            uuid: var.uuid.to_string(),
            name: var.name.to_owned(),
            value: var.value.to_owned(),
        }
    }
}

async fn list_scope_variables(
    State(state): State<RuntimeStateRef>,
    Path(scope_name): Path<String>,
) -> impl IntoResponse {
    variable_service::find_all_vars(&state.db, &scope_name)
        .await
        .map(ScopeVariableListResponse::from)
        .map(Json)
        .into_response()
}

async fn create_scope_variable(
    State(state): State<RuntimeStateRef>,
    Path(scope_name): Path<String>,
    Json(payload): Json<CreateScopeVariablePayload>,
) -> impl IntoResponse {
    variable_service::create_var(&state.db, &scope_name, &payload.name, &payload.value)
        .await
        .map(CreatedScopeVariableResponse::from)
        .map(Json)
        .into_response()
}

async fn get_scope_variable(
    State(state): State<RuntimeStateRef>,
    Path(scope_variable_path): Path<ScopeVariablePath>,
) -> impl IntoResponse {
    variable_service::find_var_by_id(&state.db, &scope_variable_path.variable_id)
        .await
        .map(|model| {
            if let Some(model) = model {
                let response: ScopeVariableResponse = model.into();
                Json(response).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        })
        .into_response()
}

async fn update_scope_variable(
    State(state): State<RuntimeStateRef>,
    Path(scope_variable_path): Path<ScopeVariablePath>,
    Json(payload): Json<UpdateScopeVariablePayload>,
) -> impl IntoResponse {
    variable_service::update_var(
        &state.db,
        &scope_variable_path.variable_id,
        payload.name.as_deref(),
        payload.value.as_deref(),
    )
    .await
    .map(|model| {
        if let Some(model) = model {
            let response: ScopeVariableResponse = model.into();
            Json(response).into_response()
        } else {
            StatusCode::NOT_FOUND.into_response()
        }
    })
    .into_response()
}

async fn delete_scope_variable(
    State(state): State<RuntimeStateRef>,
    Path(scope_variable_path): Path<ScopeVariablePath>,
) -> impl IntoResponse {
    variable_service::delete_var_by_id(&state.db, &scope_variable_path.variable_id)
        .await
        .map(|_| StatusCode::ACCEPTED)
        .into_response()
}
