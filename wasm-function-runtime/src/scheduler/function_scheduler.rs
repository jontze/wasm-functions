pub(crate) type SchedulerCache = moka::future::Cache<uuid::Uuid, uuid::Uuid>;

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
    cache: SchedulerCache,
    inner_scheduler: tokio_cron_scheduler::JobScheduler,
}

impl FunctionSchedulerImpl {
    pub(crate) async fn new() -> Self {
        let cache = moka::future::Cache::builder().build();
        let inner_scheduler = tokio_cron_scheduler::JobScheduler::new()
            .await
            .expect("Failed to create scheduler");
        Self {
            cache,
            inner_scheduler,
        }
    }
}

#[async_trait::async_trait]
impl FunctionSchedulerManagerTrait for FunctionSchedulerImpl {
    async fn add(&self, function_id: &uuid::Uuid, cron_syntax: &str) {
        let cron_job = tokio_cron_scheduler::Job::new_async(cron_syntax, |job_uuid, _lock| {
            Box::pin(async move {
                println!("{:?} Hi I ran", job_uuid);
                // TODO: Push via channel to worker or just execute the function for now
            })
        })
        .expect("Failed to setup cron job");

        let job_id = self
            .inner_scheduler
            .add(cron_job)
            .await
            .expect("Failed to add job");

        self.cache.insert(*function_id, job_id).await;
    }

    async fn remove(&self, function_id: &uuid::Uuid) {
        if let Some(job_id) = self.cache.get(function_id).await {
            self.inner_scheduler
                .remove(&job_id)
                .await
                .expect("Failed to remove function from scheduler");
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
