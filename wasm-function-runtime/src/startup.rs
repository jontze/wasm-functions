use tower_http::ServiceBuilderExt;
use tracing::info;

use crate::{
    db,
    server_state::{RuntimeState, RuntimeStateRef},
};

#[derive(Default, Clone)]
struct RequstIdGenerator {
    counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl tower_http::request_id::MakeRequestId for RequstIdGenerator {
    fn make_request_id<B>(
        &mut self,
        _: &http::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        let request_id = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            .to_string()
            .parse()
            .unwrap();

        Some(tower_http::request_id::RequestId::new(request_id))
    }
}

pub(crate) async fn run_server() {
    let db_pool = db::init_pool(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await;

    db::run_migrations(&db_pool).await;

    let runtime_state: RuntimeStateRef = std::sync::Arc::new(RuntimeState::new(db_pool));

    let app = crate::routes::create_routes()
        .with_state(runtime_state)
        .layer(
            tower::ServiceBuilder::new()
                .trace_for_http()
                .compression()
                .set_x_request_id(RequstIdGenerator::default())
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
