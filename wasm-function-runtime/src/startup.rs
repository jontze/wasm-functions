use tracing::info;

use crate::{
    db,
    server_state::{RuntimeState, RuntimeStateRef},
};

pub(crate) async fn run_server() {
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;

    db::run_migrations(&db_pool).await;

    let runtime_state: RuntimeStateRef = std::sync::Arc::new(RuntimeState::new(db_pool));

    let app = crate::routes::create_routes()
        .with_state(runtime_state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!(
        "Starting server on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
