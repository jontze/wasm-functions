pub(crate) type RuntimeStateRef = std::sync::Arc<RuntimeState>;

pub(crate) type PluginRegistry = moka::future::Cache<String, Vec<u8>>;

pub(crate) type JwkSetCache = moka::future::Cache<String, jsonwebtoken::jwk::JwkSet>;

pub(crate) struct RuntimeState {
    pub registry: PluginRegistry,
    pub jwk_cache: JwkSetCache,
    pub engine: wasmtime::Engine,
    pub db: crate::db::DbPool,
    pub app_config: crate::config::AppConfig,
    pub scheduler_manager: Box<dyn crate::scheduler::FunctionSchedulerManagerTrait>,
}

impl RuntimeState {
    pub(crate) fn new(
        db: crate::db::DbPool,
        app_config: crate::config::AppConfig,
        scheduler_manager: Box<dyn crate::scheduler::FunctionSchedulerManagerTrait>,
    ) -> Self {
        let mut config = wasmtime::Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = wasmtime::Engine::new(&config).expect("Failed to create engine");

        let registry = moka::future::Cache::builder()
            .time_to_idle(
                std::time::Duration::from_secs(60 * 60 * 24), /* 24 hours after last access */
            )
            .build();
        let jwk_cache = moka::future::Cache::builder()
            .time_to_live(std::time::Duration::from_secs(
                60 * 60 * 24, /* 24 hours after insert */
            ))
            .build();

        Self {
            registry,
            jwk_cache,
            engine,
            db,
            app_config,
            scheduler_manager,
        }
    }
}
