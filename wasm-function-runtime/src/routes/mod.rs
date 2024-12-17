use crate::{
    handlers::{api_handler, function_handler},
    server_state::RuntimeStateRef,
};

pub(crate) fn create_routes() -> axum::Router<RuntimeStateRef> {
    axum::Router::new()
        .nest("/api", api_handler::router())
        .nest("/function", function_handler::router())
}
