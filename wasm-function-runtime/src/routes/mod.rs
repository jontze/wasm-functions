use crate::{
    handlers::{api_handler, function_handler, healthz_handler},
    server_state::RuntimeStateRef,
};

pub(crate) fn create_routes(
    runtime_state: crate::server_state::RuntimeStateRef,
) -> axum::Router<RuntimeStateRef> {
    axum::Router::new()
        .nest("/api", api_handler::router(runtime_state))
        .nest("/function", function_handler::router())
        .nest("/healthz", healthz_handler::router())
}
