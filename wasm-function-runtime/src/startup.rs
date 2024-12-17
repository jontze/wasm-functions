use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::StreamExt;
use tracing::info;

use crate::{
    bindings_function_http, db,
    server_state::{RuntimeState, RuntimeStateRef},
};

pub(crate) async fn run_server() {
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;

    db::run_migrations(&db_pool).await;

    let runtime_state: RuntimeStateRef =
        std::sync::Arc::new(tokio::sync::RwLock::new(RuntimeState::new(db_pool)));

    let app = crate::routes::create_routes::<RuntimeStateRef>()
        .with_state(runtime_state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!(
        "Starting server on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    Path(path): Path<String>,
    State(state): State<RuntimeStateRef>,
) -> impl IntoResponse {
    let state = state.read().await;
    if state.registry.contains_key(&path) {
        let precompiled_bytes = state.registry.get(&path).unwrap();
        let engine = &state.engine;

        let mut http_function_builder = unsafe {
            crate::component::http::FunctionHttpBuilder::deserialize(engine, precompiled_bytes)
        };
        let http_function = http_function_builder.build().await;

        let req = bindings_function_http::Request {
            path: "/".to_string(),
            query_params: vec![],
            method: bindings_function_http::Method::Get,
            body: vec![],
            headers: vec![],
        };

        let ret = http_function
            .call_handle_request(&mut http_function_builder.store, &req)
            .await
            .expect("Failed to call function")
            .expect("Function returned a failure");

        (StatusCode::OK, [("Content-Type", "text/plain")], ret.body).into_response()
    } else {
        "Module not found".into_response()
    }
}

async fn deploy_http_function(
    State(runtime_state): State<RuntimeStateRef>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let mut body_stream = request.into_body().into_data_stream();
    let mut wasm_bytes: Vec<u8> = vec![];
    while let Some(chunk) = body_stream.next().await {
        match chunk {
            Ok(bytes) => wasm_bytes.extend(bytes),
            Err(_) => return (StatusCode::BAD_REQUEST, "Failed to read file").into_response(),
        }
    }

    let mut http_function_builder = {
        let engine = &runtime_state.read().await.engine;
        crate::component::http::FunctionHttpBuilder::from_binary(engine, &wasm_bytes)
    };

    let instance = http_function_builder.build().await;

    let path = instance
        .call_path(&mut http_function_builder.store)
        .await
        .expect("Failed to get path");

    {
        let mut state = runtime_state.write().await;
        let precombiled_binaries = http_function_builder.serialize();
        state.registry.insert(path.clone(), precombiled_binaries);
    }

    format!("Module Deployed to '/api/{path}'").into_response()
}
