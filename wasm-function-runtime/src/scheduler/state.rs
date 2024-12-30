pub(crate) type SchedulerCache = moka::future::Cache<uuid::Uuid, uuid::Uuid>;

pub(crate) type BinaryCache = moka::future::Cache<uuid::Uuid, Vec<u8>>;

pub(crate) struct SchedulerState {
    pub db_pool: crate::db::DbPool,
    pub engine: wasmtime::Engine,
    pub cache: SchedulerCache,
    pub binary_cache: BinaryCache,
}

impl SchedulerState {
    pub(crate) async fn new(db_pool: crate::db::DbPool, engine: wasmtime::Engine) -> Self {
        let cache = moka::future::Cache::builder().build();
        let binary_cache = moka::future::Cache::builder()
            .time_to_idle(std::time::Duration::from_secs(
                60 * 10, /* 10 Minutes cache */
            ))
            .build();
        Self {
            db_pool,
            engine,
            cache,
            binary_cache,
        }
    }
}
