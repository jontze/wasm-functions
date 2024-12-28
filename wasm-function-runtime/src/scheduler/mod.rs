use function_scheduler::FunctionSchedulerExecutorTrait;

pub(crate) mod function_scheduler;

pub(crate) use function_scheduler::{FunctionSchedulerImpl, FunctionSchedulerManagerTrait};

use crate::services::function_service;

pub(crate) async fn run_scheduler<TScheduler>(scheduler: &TScheduler, db_pool: &crate::db::DbPool)
where
    TScheduler: FunctionSchedulerManagerTrait + FunctionSchedulerExecutorTrait,
{
    let funcs = function_service::find_all_scheduled_func(db_pool).await;

    for func in funcs {
        scheduler.add(&func.uuid, &func.cron).await;
    }

    scheduler.run().await;
}
