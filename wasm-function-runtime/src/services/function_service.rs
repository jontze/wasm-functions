use sea_orm::{prelude::*, Set};
use std::{ops::Deref, path::Path};
use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    db::DbPool,
    domain::{self, function::WasmFunctionTrait},
    handlers::api_handler::{CreateHttpFunctionPayload, CreateScheduledFunctionPayload},
};

const WASM_FUNCTIONS_DIR: &str = "wasm_functions";

pub(crate) async fn find_http_func(
    db_pool: &DbPool,
    function_path: &str,
    function_method: &str,
) -> Option<(domain::function::HttpFunction, Vec<u8>)> {
    let http_function = entity::http_function::Entity::find()
        .filter(entity::http_function::Column::Path.eq(format!("/{}", function_path)))
        .filter(entity::http_function::Column::Method.eq(function_method))
        .one(db_pool.deref())
        .await
        .unwrap();

    if let Some(http_function) = http_function.map(domain::function::HttpFunction::from) {
        let file_name = http_function.related_wasm();
        Some((http_function, extract_wasm_file_bytes(&file_name).await))
    } else {
        None
    }
}

pub(crate) async fn create_http_func(
    db_pool: &DbPool,
    payload: CreateHttpFunctionPayload,
) -> domain::function::HttpFunction {
    let transaction = db_pool.start_transaction().await;

    let scope = crate::services::scope_service::create_or_find_scope(&transaction, "default").await;

    let http_function = entity::http_function::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        method: Set(payload.method),
        path: Set(payload.path),
        is_public: Set(payload.is_public),
        scope_id: Set(scope.uuid),
    };

    let http_function: entity::http_function::Model = http_function
        .insert(transaction.deref())
        .await
        .expect("Failed to insert http function");

    let http_function: domain::function::HttpFunction = http_function.into();

    store_wasm_file(payload.wasm_bytes, &http_function.related_wasm()).await;

    transaction.commit().await;
    http_function
}

pub(crate) async fn create_scheduled_func(
    db_pool: &DbPool,
    payload: CreateScheduledFunctionPayload,
) -> domain::function::ScheduledFunction {
    todo!("Save and return the scheduled function")
}

async fn store_wasm_file(bytes: Vec<u8>, target_file_name: &str) {
    let file_path = Path::new(WASM_FUNCTIONS_DIR).join(target_file_name);

    // Create the folder if it doesn't exist
    if !file_path.parent().unwrap().exists() {
        tokio::fs::create_dir_all(file_path.parent().unwrap())
            .await
            .unwrap();
    }

    // Create the file and write the bytes
    let mut file = TokioFile::create(file_path).await.unwrap();
    file.write_all(&bytes).await.unwrap();
    file.flush().await.expect("Failed to sync file");
}

async fn extract_wasm_file_bytes(file_name: &str) -> Vec<u8> {
    let file_path = Path::new(WASM_FUNCTIONS_DIR).join(file_name);

    let mut file = TokioFile::open(file_path).await.unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).await.unwrap();
    bytes
}
