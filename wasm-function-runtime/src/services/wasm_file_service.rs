use std::path::Path;
use tokio::{
    fs::File as TokioFile,
    io::{AsyncReadExt, AsyncWriteExt},
};

const WASM_FUNCTIONS_DIR: &str = "wasm_functions";

pub(crate) async fn store_file(bytes: Vec<u8>, target_file_name: &str) {
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

pub(crate) async fn extract_file_bytes(file_name: &str) -> Vec<u8> {
    let file_path = Path::new(WASM_FUNCTIONS_DIR).join(file_name);

    let mut file = TokioFile::open(file_path).await.unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).await.unwrap();
    bytes
}
