pub(crate) type RuntimeStateRef = std::sync::Arc<RuntimeState>;

pub(crate) type JwkSetCache = moka::future::Cache<String, jsonwebtoken::jwk::JwkSet>;

pub(crate) struct RuntimeState {
    pub jwk_cache: JwkSetCache,
    pub engine: wasmtime::Engine,
    pub db: crate::db::DbPool,
    pub app_config: crate::config::AppConfig,
    pub scheduler_manager: Box<dyn crate::scheduler::FunctionSchedulerManagerTrait>,
    pub storage_backend: std::sync::Arc<dyn crate::storage::StorageBackend>,
    pub cache_backend: std::sync::Arc<dyn crate::cache::CacheBackend>,
}

impl RuntimeState {
    pub(crate) fn new(
        db: crate::db::DbPool,
        wasm_engine: wasmtime::Engine,
        app_config: crate::config::AppConfig,
        scheduler_manager: Box<dyn crate::scheduler::FunctionSchedulerManagerTrait>,
        storage_backend: std::sync::Arc<dyn crate::storage::StorageBackend>,
        cache_backend: std::sync::Arc<dyn crate::cache::CacheBackend>,
    ) -> Self {
        let jwk_cache = moka::future::Cache::builder()
            .time_to_live(std::time::Duration::from_secs(
                60 * 60 * 24, /* 24 hours after insert */
            ))
            .build();

        Self {
            jwk_cache,
            engine: wasm_engine,
            db,
            app_config,
            scheduler_manager,
            storage_backend,
            cache_backend,
        }
    }
}
