use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

pub(crate) mod http;
pub(crate) mod scheduled;

pub(crate) fn setup_engine() -> wasmtime::Engine {
    let mut config = wasmtime::Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    wasmtime::Engine::new(&config).expect("Failed to create engine")
}

pub(crate) struct ComponentState {
    ctx: WasiCtx,
    http_ctx: wasmtime_wasi_http::WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for ComponentState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl wasmtime_wasi_http::WasiHttpView for ComponentState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
