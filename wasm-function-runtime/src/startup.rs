use tower_http::ServiceBuilderExt;
use tracing::info;

use crate::{
    component,
    config::Loader,
    db, scheduler,
    server_state::{RuntimeState, RuntimeStateRef},
};

pub(crate) async fn run_server() {
    // Load application configuration
    let app_config = crate::config::AppConfig::load();

    // Setup database connection pool and run migrations
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;
    db::run_migrations(&db_pool).await;

    // Setup Cache based on configuration
    let cache_backend: std::sync::Arc<dyn crate::cache::CacheBackend> =
        if let Some(redis_config) = &app_config.redis_cache {
            std::sync::Arc::new(crate::cache::RedisCache::new(&redis_config.connection_str).await)
        } else {
            std::sync::Arc::new(crate::cache::LocalCache::default())
        };

    // Setup storage backend based on configuration
    let storage_backend: Box<dyn crate::storage::StorageBackend> =
        if let Some(minio_config) = &app_config.minio_storage {
            Box::new(crate::storage::GeneralS3::new_minio(
                &minio_config.endpoint,
                &minio_config.access_key,
                &minio_config.secret_key,
                &minio_config.bucket,
            ))
        } else if let Some(azure_config) = &app_config.azure_storage {
            Box::new(crate::storage::GeneralS3::new_azure(
                &azure_config.account,
                &azure_config.access_key,
                &azure_config.container,
            ))
        } else if let Some(hetzner_config) = &app_config.hetzner_storage {
            Box::new(crate::storage::GeneralS3::new_hetzner(
                &hetzner_config.access_key,
                &hetzner_config.secret_key,
                &hetzner_config.bucket_url,
                &hetzner_config.bucket_name,
                &hetzner_config.region,
            ))
        } else {
            Box::new(if let Some(storage_dir) = &app_config.local_storage_dir {
                crate::storage::file_system::FileSystemStorage::new(storage_dir)
            } else {
                crate::storage::file_system::FileSystemStorage::default()
            })
        };
    let storage_backend = std::sync::Arc::new(crate::storage::CachedStorage::new(
        storage_backend,
        cache_backend.clone(),
    ));

    // Setup WASI engine
    let wasm_engine = component::setup_engine();

    // Setup function scheduler
    let func_scheduler = scheduler::FunctionSchedulerImpl::new(
        db_pool.clone(),
        wasm_engine.clone(),
        storage_backend.clone(),
    )
    .await;
    scheduler::run_scheduler(&func_scheduler, &db_pool).await;

    // Init server state
    let runtime_state: RuntimeStateRef = std::sync::Arc::new(RuntimeState::new(
        db_pool,
        wasm_engine,
        app_config,
        Box::new(func_scheduler),
        storage_backend,
        cache_backend,
    ));

    // Setup server with handlers and middlewares
    let app = crate::routes::create_routes(runtime_state.clone())
        .with_state(runtime_state)
        .layer(
            tower::ServiceBuilder::new()
                .trace_for_http()
                .compression()
                .set_x_request_id(crate::middlewares::request_id::RequstIdGenerator::default())
                .propagate_x_request_id()
                .catch_panic()
                .trim_trailing_slash(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!(
        "Starting server on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
