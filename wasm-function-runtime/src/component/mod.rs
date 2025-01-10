use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

pub(crate) mod http;
pub(crate) mod scheduled;

pub(crate) fn setup_engine() -> wasmtime::Engine {
    let mut config = wasmtime::Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    wasmtime::Engine::new(&config).expect("Failed to create engine")
}

pub(crate) struct ComponentStateBuilder {
    ctx: WasiCtxBuilder,
    http_ctx: wasmtime_wasi_http::WasiHttpCtx,
    table: ResourceTable,
}

impl ComponentStateBuilder {
    pub fn new() -> Self {
        Self {
            ctx: WasiCtxBuilder::new(),
            http_ctx: wasmtime_wasi_http::WasiHttpCtx::new(),
            table: ResourceTable::new(),
        }
    }

    pub fn with_envs(&mut self, env: &[(impl AsRef<str>, impl AsRef<str>)]) -> &mut Self {
        self.ctx.envs(env);
        self
    }

    pub fn build(mut self) -> ComponentState {
        let ctx = self.ctx.build();

        ComponentState {
            ctx,
            http_ctx: self.http_ctx,
            table: self.table,
        }
    }
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
