use tower_http::ServiceBuilderExt;
use tracing::info;

use crate::{
    component,
    config::Loader,
    db, scheduler,
    server_state::{RuntimeState, RuntimeStateRef},
};

pub(crate) async fn run_server() {
    // Setup database connection pool and run migrations
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;
    db::run_migrations(&db_pool).await;

    // Setup storage backend and function cache
    let mut storage_backend = std::sync::Arc::new(crate::storage::CachedStorage::default());
    let function_cache = std::sync::Arc::new(crate::function_cache::LocalFunctionCache::default());

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

    // Load application configuration
    let app_config = crate::config::AppConfig::load();

    // If Minio storage is configured, use it instead of the default file system storage
    if let Some(minio_config) = &app_config.minio_storage {
        let minio_storage_backend = Box::new(crate::storage::GeneralS3::new(
            &minio_config.endpoint,
            &minio_config.access_key,
            &minio_config.secret_key,
            &minio_config.bucket,
        ));
        storage_backend =
            std::sync::Arc::new(crate::storage::CachedStorage::new(minio_storage_backend));
    }

    // Init server state
    let runtime_state: RuntimeStateRef = std::sync::Arc::new(RuntimeState::new(
        db_pool,
        wasm_engine,
        app_config,
        Box::new(func_scheduler),
        storage_backend,
        function_cache,
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
