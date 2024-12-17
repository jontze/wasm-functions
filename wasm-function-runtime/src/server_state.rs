pub(crate) type RuntimeStateRef = std::sync::Arc<tokio::sync::RwLock<RuntimeState>>;

pub(crate) type PluginRegistry = std::collections::HashMap<String, Vec<u8>>;

pub(crate) struct RuntimeState {
    pub registry: PluginRegistry,
    pub config: wasmtime::Config,
    pub engine: wasmtime::Engine,
    pub db: crate::db::DbPool,
}

impl RuntimeState {
    pub(crate) fn new(db: crate::db::DbPool) -> Self {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = wasmtime::Engine::new(&config).expect("Failed to create engine");
        Self {
            registry: std::collections::HashMap::new(),
            config,
            engine,
            db,
        }
    }
}
