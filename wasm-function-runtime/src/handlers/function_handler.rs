use axum::{extract::Path, response::IntoResponse, routing::method_routing::get};

use crate::server_state::RuntimeStateRef;

pub(crate) fn router() -> axum::Router<RuntimeStateRef> {
    axum::Router::new().route("/:path", get(handle_request))
}

async fn handle_request(Path(path): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", path).into_response()
}
