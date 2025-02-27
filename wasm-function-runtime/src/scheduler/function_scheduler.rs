use tracing::{debug, error};
use wasmtime::Store;

use crate::{
    bindings_function_scheduled,
    component::ComponentState,
    services::{function_service, variable_service},
};

#[async_trait::async_trait]
pub(crate) trait FunctionSchedulerManagerTrait: Send + Sync {
    async fn add(&self, function_id: &uuid::Uuid, cron_syntax: &str);
    async fn remove(&self, function_id: &uuid::Uuid);
}

#[async_trait::async_trait]
pub(crate) trait FunctionSchedulerExecutorTrait {
    async fn run(&self);
}

pub(crate) struct FunctionSchedulerImpl {
    inner_scheduler: tokio_cron_scheduler::JobScheduler,
    state: crate::scheduler::state::SchedulerState,
}

impl FunctionSchedulerImpl {
    pub(crate) async fn new(
        db_pool: crate::db::DbPool,
        wasm_engine: wasmtime::Engine,
        storage_backend: std::sync::Arc<dyn crate::storage::StorageBackend>,
    ) -> Self {
        let inner_scheduler = tokio_cron_scheduler::JobScheduler::new()
            .await
            .expect("Failed to create scheduler");
        let state =
            crate::scheduler::state::SchedulerState::new(db_pool, wasm_engine, storage_backend)
                .await;
        Self {
            inner_scheduler,
            state,
        }
    }
}

#[async_trait::async_trait]
impl FunctionSchedulerManagerTrait for FunctionSchedulerImpl {
    async fn add(&self, function_id: &uuid::Uuid, cron_syntax: &str) {
        // Prepare variables to move into the job
        let db_pool = self.state.db_pool.clone();
        let engine = self.state.engine.clone();
        let function_id = *function_id;
        let binary_cache = self.state.binary_cache.clone();
        let storage_backend = self.state.storage_backend.clone();

        let cron_job = tokio_cron_scheduler::Job::new_async(cron_syntax, move |job_uuid, _lock| {
            // Prepare variables to move into the async block
            let db_pool = db_pool.clone();
            let engine = engine.clone();
            let binary_cache = binary_cache.clone();
            let storage_backend = storage_backend.clone();

            Box::pin(async move {
                debug!("Execute scheduled function '{function_id}' ({job_uuid})",);
                // Extract function variables
                let funct_vars =
                    variable_service::find_vars_by_scheduled_func_id(&db_pool, &function_id)
                        .await
                        .expect("Failed to find variables for function")
                        .expect("Failed to find variables for function");

                // Check if the function is in the cache
                let (func, mut func_store): (
                    bindings_function_scheduled::FunctionScheduled,
                    Store<ComponentState>,
                ) = if let Some(cached_serialized_bytes) = binary_cache.get(&function_id).await {
                    // If it is, deserialize the function from the cache and execute
                    let mut func_builder = unsafe {
                        crate::component::scheduled::FunctionScheduledBuilder::deserialize(
                            &engine,
                            &cached_serialized_bytes,
                        )
                    };

                    // Add the variables to the function store
                    func_builder = func_builder.with_variables(&funct_vars);

                    Some(func_builder.build().await)
                } else {
                    // Otherwise, fetch the function from the database
                    if let Some((_, bytes)) = function_service::find_scheduled_func(
                        &db_pool,
                        &*storage_backend,
                        &function_id,
                    )
                    .await
                    .expect("Failed to find function")
                    {
                        // Exract the function from the storage
                        let mut func_builder =
                            crate::component::scheduled::FunctionScheduledBuilder::from_binary(
                                &engine, &bytes,
                            );

                        // Serialize the function to the cache to speed up further executions
                        let serialized_bytes = func_builder.serialize();
                        binary_cache
                            .insert(function_id.to_owned(), serialized_bytes)
                            .await;

                        // Add the variables to the function store
                        func_builder = func_builder.with_variables(&funct_vars);

                        Some(func_builder.build().await)
                    } else {
                        None
                    }
                }
                .expect("Failed to find function");

                // Execute the function
                match func
                    .call_run_job(&mut func_store)
                    .await
                    .expect("Failed to call function")
                {
                    Ok(_) => {
                        debug!("Scheduled function executed successfully");
                    }
                    Err(e) => {
                        error!("Scheduled function failed: {e:?}");
                    }
                };
            })
        })
        .expect("Failed to setup cron job");

        let job_id = self
            .inner_scheduler
            .add(cron_job)
            .await
            .expect("Failed to add job");

        self.state
            .cache
            .insert(function_id.to_owned(), job_id)
            .await;
    }

    async fn remove(&self, function_id: &uuid::Uuid) {
        if let Some(job_id) = self.state.cache.remove(function_id).await {
            self.inner_scheduler
                .remove(&job_id)
                .await
                .expect("Failed to remove function from scheduler");
            self.state.binary_cache.remove(function_id).await;
        }
    }
}

#[async_trait::async_trait]
impl FunctionSchedulerExecutorTrait for FunctionSchedulerImpl {
    async fn run(&self) {
        self.inner_scheduler
            .start()
            .await
            .expect("Failed to start scheduler");
    }
}
