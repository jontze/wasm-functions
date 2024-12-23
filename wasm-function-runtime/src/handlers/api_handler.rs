use axum::{extract::State, response::IntoResponse, routing::post};

use crate::{domain, server_state::RuntimeStateRef, services::function_service};

pub(crate) fn router(
    app_state: crate::server_state::RuntimeStateRef,
) -> axum::routing::Router<RuntimeStateRef> {
    axum::Router::new()
        .route("/deploy", post(deploy_function_with_manifest))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state,
            crate::middlewares::auth::auth,
        ))
}

#[derive(Default)]
pub(crate) struct CreateHttpFunctionPayload {
    pub name: String,
    pub method: String,
    pub path: String,
    pub wasm_bytes: Vec<u8>,
}

async fn deploy_function_with_manifest(
    State(state): axum::extract::State<RuntimeStateRef>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let mut payload = CreateHttpFunctionPayload::default();

    while let Some(field) = multipart.next_field().await.expect("Failed to read file") {
        match field.file_name().expect("Failed to get file name") {
            "manifest.toml" => {
                let data = field.bytes().await.expect("Failed to read field");
                let manifest = toml::from_str::<domain::manifest::Manifest>(
                    std::str::from_utf8(&data).expect("Failed to parse manifest"),
                )
                .expect("Failed to parse manifest");

                payload.name = manifest.name;
                payload.method = manifest.method;
                payload.path = manifest.path;
            }
            file_name => {
                if file_name.ends_with(".wasm") {
                    let data = field.bytes().await.expect("Failed to read field");
                    payload.wasm_bytes = data.to_vec();
                } else {
                    panic!("Invalid file name: {}", file_name);
                }
            }
        };
    }

    function_service::create_http_func(&state.db, payload).await;

    "OK".into_response()
}
