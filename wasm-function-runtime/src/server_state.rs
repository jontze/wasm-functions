pub(crate) type RuntimeStateRef = std::sync::Arc<RuntimeState>;

pub(crate) type PluginRegistry = moka::future::Cache<String, Vec<u8>>;

pub(crate) struct RuntimeState {
    pub registry: PluginRegistry,
    pub engine: wasmtime::Engine,
    pub db: crate::db::DbPool,
}

impl RuntimeState {
    pub(crate) fn new(db: crate::db::DbPool) -> Self {
        let mut config = wasmtime::Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = wasmtime::Engine::new(&config).expect("Failed to create engine");
        let registry = moka::future::Cache::builder().build();
        Self {
            registry,
            engine,
            db,
        }
    }
}
