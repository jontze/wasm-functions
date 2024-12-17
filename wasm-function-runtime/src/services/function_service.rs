use sea_orm::{prelude::*, Set, TransactionTrait};
use std::{ops::Deref, path::Path};
use tokio::{fs::File as TokioFile, io::AsyncWriteExt};

use crate::{
    db::DbPool,
    domain::{self, function::WasmFunctionTrait},
    handlers::api_handler::CreateHttpFunctionPayload,
};

pub(crate) async fn create(
    db_pool: &DbPool,
    payload: CreateHttpFunctionPayload,
) -> domain::function::HttpFunction {
    let http_function = entity::http_function::ActiveModel {
        name: Set(payload.name),
        method: Set(payload.method),
        path: Set(payload.path),
        id: Set(Uuid::new_v4()),
    };
    let transaction = db_pool.deref().begin().await.unwrap();

    let http_function: entity::http_function::Model =
        http_function.insert(&transaction).await.unwrap();

    let http_function: domain::function::HttpFunction = http_function.into();

    store_wasm_file(payload.wasm_bytes, &http_function.related_wasm()).await;

    transaction.commit().await.unwrap();
    http_function.into()
}

async fn store_wasm_file<'a>(bytes: Vec<u8>, target_file_name: &'a str) -> () {
    let file_path = Path::new("wasm_functions").join(target_file_name);

    let mut file = TokioFile::create(file_path).await.unwrap();
    file.write_all(&bytes).await.unwrap();
    file.flush().await.expect("Failed to sync file");
}
