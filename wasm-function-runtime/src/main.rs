use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::StreamExt;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;
use wasmtime::{
    component::{Component, Instance, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

type PluginRegistry = Arc<RwLock<HashMap<String, Instance>>>;

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

#[tokio::main]
async fn main() {
    let registry: PluginRegistry = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/:path", get(handle_request))
        .route("/deploy", post(deploy_module))
        .with_state(registry);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    Path(path): Path<String>,
    state: State<PluginRegistry>,
) -> impl IntoResponse {
    let registry = state.deref().read().await;
    if registry.get(&path).is_some() {
        "Module executed"
    } else {
        "Module not found"
    }
}

async fn deploy_module(State(_): State<PluginRegistry>, request: Request) -> impl IntoResponse {
    let mut body_stream = request.into_body().into_data_stream();

    let mut wasm_bytes: Vec<u8> = vec![];
    while let Some(chunk) = body_stream.next().await {
        match chunk {
            Ok(bytes) => wasm_bytes.extend(bytes),
            Err(_) => return (StatusCode::BAD_REQUEST, "Failed to read file").into_response(),
        }
    }

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("Failed to create engine");

    let component = match Component::from_binary(&engine, &wasm_bytes) {
        Ok(component) => component,
        Err(_) => return (StatusCode::BAD_REQUEST, "Failed to create component").into_response(),
    };

    let res_table = ResourceTable::new();
    let wasi_ctx = WasiCtxBuilder::new().inherit_env().build();

    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi_ctx,
            table: res_table,
        },
    );
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Failed to add WASI to linker");

    let instance = linker
        .instantiate(&mut store, &component)
        .expect("Failed to instantiate module");

    let hello_world_func = instance
        .get_typed_func::<(), (String,)>(&mut store, "hello-world")
        .expect("Failed to get function");

    let ret = hello_world_func
        .call(&mut store, ())
        .expect("Failed to call function");

    println!("Result: {:?}", ret);

    //println!("Path: {:?}", path);

    "Module Deployed".into_response()
}
