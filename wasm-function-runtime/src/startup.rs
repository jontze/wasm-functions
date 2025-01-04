use tower_http::ServiceBuilderExt;
use tracing::info;

use crate::{
    component,
    config::Loader,
    db, scheduler,
    server_state::{RuntimeState, RuntimeStateRef},
};

pub(crate) async fn run_server() {
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;
    db::run_migrations(&db_pool).await;

    let storage_backend = std::sync::Arc::new(crate::storage::FileSystemStorage::default());

    let wasm_engine = component::setup_engine();

    let func_scheduler = scheduler::FunctionSchedulerImpl::new(
        db_pool.clone(),
        wasm_engine.clone(),
        storage_backend.clone(),
    )
    .await;
    scheduler::run_scheduler(&func_scheduler, &db_pool).await;

    let app_config = crate::config::AppConfig::load();

    let runtime_state: RuntimeStateRef = std::sync::Arc::new(RuntimeState::new(
        db_pool,
        wasm_engine,
        app_config,
        Box::new(func_scheduler),
        storage_backend,
    ));

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
