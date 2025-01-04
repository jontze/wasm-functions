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
    pub storage_backend: std::sync::Arc<dyn crate::storage::StorageBackend>,
}

impl RuntimeState {
    pub(crate) fn new(
        db: crate::db::DbPool,
        wasm_engine: wasmtime::Engine,
        app_config: crate::config::AppConfig,
        scheduler_manager: Box<dyn crate::scheduler::FunctionSchedulerManagerTrait>,
        storage_backend: std::sync::Arc<dyn crate::storage::StorageBackend>,
    ) -> Self {
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
            engine: wasm_engine,
            db,
            app_config,
            scheduler_manager,
            storage_backend,
        }
    }
}
