use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod component;
pub(crate) mod config;
mod db;
pub(crate) mod domain;
pub(crate) mod handlers;
pub(crate) mod middlewares;
mod routes;
pub(crate) mod scheduler;
pub(crate) mod server_state;
pub(crate) mod services;
pub(crate) mod startup;
pub(crate) mod storage;

pub(crate) mod bindings_function_http {
    wasmtime::component::bindgen!({
        world: "function-http",
         path: "./wit-http/",
        async: true,
    });
}

pub(crate) mod bindings_function_scheduled {
    wasmtime::component::bindgen!({
        world: "function-scheduled",
        path: "./wit-scheduled/",
        async: true
    });
}

#[tokio::main]
async fn main() {
    // Initialize logging/tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().expect("Failed to create filter"),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Start the server
    startup::run_server().await;
}
