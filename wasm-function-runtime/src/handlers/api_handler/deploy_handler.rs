use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};

use super::{domain, function_service, RuntimeStateRef};

pub(super) fn router() -> Router<RuntimeStateRef> {
    Router::new().route("/", post(deploy_function_with_manifest))
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

                    function_service::create_http_func(&state.db, &*state.storage_backend, payload)
                        .await
                        .map_err(|e| e.into_response())?;
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
                        &*state.storage_backend,
                        payload,
                    )
                    .await
                    .map_err(|e| e.into_response())?;
                } else {
                    return Err("Scheduled function must have scheduled section".into_response());
                }
            }
        };
    } else {
        return Err("Manifest file is required".into_response());
    }

    Ok(StatusCode::CREATED)
}
