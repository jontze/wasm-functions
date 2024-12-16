use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use wasmtime::{Config, Engine};

mod component;

mod bindings_function_http {
    wasmtime::component::bindgen!({
        world: "function-http",
        async: true
    });
}

type PluginRegistry = HashMap<String, Vec<u8>>;

struct RuntimeState {
    registry: PluginRegistry,
    config: Config,
    engine: Engine,
}

type RuntimeStateRef = Arc<RwLock<RuntimeState>>;

impl RuntimeState {
    fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config).expect("Failed to create engine");
        Self {
            registry: HashMap::new(),
            config,
            engine,
        }
    }
}

#[tokio::main]
async fn main() {
    let runtime_state: RuntimeStateRef = Arc::new(RwLock::new(RuntimeState::new()));

    let app = Router::new()
        .route("/api/:path", get(handle_request))
        .route("/deploy", post(deploy_http_function))
        .with_state(runtime_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

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

        let mut http_function_builder =
            unsafe { component::http::FunctionHttpBuilder::deserialize(engine, precompiled_bytes) };
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
    request: Request,
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
        component::http::FunctionHttpBuilder::from_binary(engine, &wasm_bytes)
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
