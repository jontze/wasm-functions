use axum::{extract::Path, response::IntoResponse, routing::method_routing::get};

pub(crate) fn router<TState>() -> axum::Router<TState>
where
    TState: Clone + Send + Sync + 'static,
{
    axum::Router::<TState>::new().route("/:path", get(handle_request))
}

async fn handle_request(Path(path): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", path).into_response()
}
