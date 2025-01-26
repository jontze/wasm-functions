use sea_orm::{prelude::*, IntoActiveModel, Set};
use std::ops::Deref;

use crate::{
    db::DbPool,
    domain::{self, function::WasmFunctionTrait},
    handlers::api_handler::{CreateHttpFunctionPayload, CreateScheduledFunctionPayload},
    services::scope_service,
    storage,
};

use super::errors::ServiceError;

pub(crate) async fn find_http_func_by_scope_and_req(
    db_pool: &DbPool,
    scope_name: &str,
    function_path: &str,
    function_method: &str,
) -> Result<Option<domain::function::HttpFunction>, ServiceError> {
    let scope = match entity::scope::Entity::find()
        .filter(entity::scope::Column::Name.eq(scope_name))
        .one(db_pool)
        .await?
    {
        Some(scope) => scope,
        None => return Ok(None),
    };

    let path = if function_path.starts_with('/') {
        function_path.to_string()
    } else {
        format!("/{}", function_path)
    };

    let http_function = entity::http_function::Entity::find()
        .filter(entity::http_function::Column::Path.eq(&path))
        .filter(entity::http_function::Column::Method.eq(function_method))
        .filter(entity::http_function::Column::ScopeId.eq(scope.id))
        .one(db_pool)
        .await?;

    Ok(http_function.map(domain::function::HttpFunction::from))
}

pub(crate) async fn find_all_funcs(
    db_pool: &DbPool,
    scope_name: &str,
) -> Result<Vec<domain::function::Function>, ServiceError> {
    if let Some(scope) = scope_service::get_scope_by_name(db_pool, scope_name).await? {
        // Extract http functions
        let http_fuctions: Vec<domain::function::HttpFunction> =
            entity::http_function::Entity::find()
                .filter(entity::http_function::Column::ScopeId.eq(scope.uuid))
                .all(db_pool)
                .await?
                .into_iter()
                .map(|model| model.into())
                .collect();

        // Extract scheduled functions
        let scheduled_functions: Vec<domain::function::ScheduledFunction> =
            entity::scheduled_function::Entity::find()
                .filter(entity::scheduled_function::Column::ScopeId.eq(scope.uuid))
                .all(db_pool)
                .await?
                .into_iter()
                .map(|model| model.into())
                .collect();

        // Merge the functions into a single vector
        let mut functions: Vec<domain::function::Function> = http_fuctions
            .into_iter()
            .map(domain::function::Function::Http)
            .collect();
        functions.extend(
            scheduled_functions
                .into_iter()
                .map(domain::function::Function::Scheduled),
        );

        // Sort functions by name
        functions.sort_by(|a, b| a.name().cmp(b.name()));

        // Return the merged and sorted functions
        Ok(functions)
    } else {
        Ok(vec![])
    }
}

pub(crate) async fn find_all_scheduled_func(
    db_pool: &DbPool,
) -> Result<Vec<domain::function::ScheduledFunction>, ServiceError> {
    Ok(entity::scheduled_function::Entity::find()
        .all(db_pool)
        .await?
        .into_iter()
        .map(|model| model.into())
        .collect())
}

pub(crate) async fn delete_http_func(
    db_pool: &DbPool,
    func_cache: &dyn crate::function_cache::FunctionCacheBackend,
    storage_backend: &dyn storage::StorageBackend,
    function_id: &uuid::Uuid,
) -> Result<(), ServiceError> {
    let http_function = entity::http_function::Entity::find()
        .filter(entity::http_function::Column::Id.eq(*function_id))
        .one(db_pool)
        .await?;

    if let Some(http_function) = http_function {
        http_function.clone().delete(db_pool).await?;

        let http_function: domain::function::HttpFunction = http_function.into();
        storage_backend
            .delete_file(&http_function.related_wasm())
            .await?;

        func_cache.invalidate(&http_function.related_wasm()).await;
    }
    Ok(())
}

pub(crate) async fn find_scheduled_func(
    db_pool: &DbPool,
    storage_backend: &dyn storage::StorageBackend,
    function_id: &uuid::Uuid,
) -> Result<Option<(domain::function::ScheduledFunction, Vec<u8>)>, ServiceError> {
    let func: Option<domain::function::ScheduledFunction> =
        entity::scheduled_function::Entity::find()
            .filter(entity::scheduled_function::Column::Id.eq(*function_id))
            .one(db_pool)
            .await?
            .map(|model| model.into());

    if let Some(func) = func {
        let file_name = func.related_wasm();
        Ok(Some((
            func,
            storage_backend.extract_file_bytes(&file_name).await?,
        )))
    } else {
        Ok(None)
    }
}

pub(crate) async fn delete_scheduled_func(
    db_pool: &DbPool,
    cache: &dyn crate::scheduler::FunctionSchedulerManagerTrait,
    storage_backend: &dyn storage::StorageBackend,
    function_id: &uuid::Uuid,
) -> Result<(), ServiceError> {
    let scheduled_function = entity::scheduled_function::Entity::find()
        .filter(entity::scheduled_function::Column::Id.eq(*function_id))
        .one(db_pool)
        .await?;

    if let Some(scheduled_function) = scheduled_function {
        scheduled_function.clone().delete(db_pool).await?;

        let scheduled_function: domain::function::ScheduledFunction = scheduled_function.into();
        storage_backend
            .delete_file(&scheduled_function.related_wasm())
            .await?;

        cache.remove(&scheduled_function.uuid).await;
    }
    Ok(())
}

pub(crate) async fn create_http_func(
    db_pool: &DbPool,
    storage_backend: &dyn crate::storage::StorageBackend,
    payload: CreateHttpFunctionPayload,
) -> Result<domain::function::HttpFunction, ServiceError> {
    let transaction = db_pool.start_transaction().await;

    let scope =
        crate::services::scope_service::create_or_find_scope(&transaction, &payload.scope).await?;

    let http_function: domain::function::HttpFunction = match entity::http_function::Entity::find()
        .filter(entity::http_function::Column::ScopeId.eq(scope.uuid))
        .filter(entity::http_function::Column::Name.eq(&payload.name))
        .one(transaction.deref())
        .await?
    {
        Some(existing_http_function) => {
            let mut existing_http_function = existing_http_function.into_active_model();
            existing_http_function.method = Set(payload.method);
            existing_http_function.scope_id = Set(scope.uuid);
            existing_http_function.path = Set(payload.path);
            existing_http_function.is_public = Set(payload.is_public);

            existing_http_function.update(transaction.deref()).await?
        }
        None => {
            entity::http_function::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(payload.name),
                method: Set(payload.method),
                path: Set(payload.path),
                is_public: Set(payload.is_public),
                scope_id: Set(scope.uuid),
                content_hash: Set(domain::function::Function::hash(&payload.wasm_bytes)),
            }
            .insert(transaction.deref())
            .await?
        }
    }
    .into();

    storage_backend
        .store_file(payload.wasm_bytes, &http_function.related_wasm())
        .await?;

    transaction.commit().await;
    Ok(http_function)
}

pub(crate) async fn create_scheduled_func(
    db_pool: &DbPool,
    func_scheduler: &dyn crate::scheduler::FunctionSchedulerManagerTrait,
    storage_backend: &dyn crate::storage::StorageBackend,
    payload: CreateScheduledFunctionPayload,
) -> Result<domain::function::ScheduledFunction, ServiceError> {
    let transaction = db_pool.start_transaction().await;

    let scope =
        crate::services::scope_service::create_or_find_scope(&transaction, &payload.scope).await?;

    let mut previous_scheduled_func: Option<entity::scheduled_function::Model> = None;

    let scheduled_function: domain::function::ScheduledFunction =
        match entity::scheduled_function::Entity::find()
            .filter(entity::scheduled_function::Column::ScopeId.eq(scope.uuid))
            .filter(entity::scheduled_function::Column::Name.eq(&payload.name))
            .one(transaction.deref())
            .await?
        {
            Some(existing_scheduled_func) => {
                previous_scheduled_func = Some(existing_scheduled_func.clone());

                let mut existing_scheduled_func = existing_scheduled_func.into_active_model();
                existing_scheduled_func.scope_id = Set(scope.uuid);
                existing_scheduled_func.cron = Set(payload.cron);

                existing_scheduled_func.update(transaction.deref()).await?
            }
            None => {
                // No existing scheduled func, so we need to create one
                entity::scheduled_function::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    name: Set(payload.name),
                    cron: Set(payload.cron),
                    scope_id: Set(scope.uuid),
                    content_hash: Set(domain::function::Function::hash(&payload.wasm_bytes)),
                }
                .insert(transaction.deref())
                .await?
            }
        }
        .into();

    storage_backend
        .store_file(payload.wasm_bytes, &scheduled_function.related_wasm())
        .await?;

    transaction.commit().await;

    if let Some(previous_scheduled_func) = previous_scheduled_func {
        func_scheduler.remove(&previous_scheduled_func.id).await;
    }

    func_scheduler
        .add(&scheduled_function.uuid, &scheduled_function.cron)
        .await;

    Ok(scheduled_function)
}
