use crate::handlers::{api_handler, function_handler};

pub(crate) fn create_routes<TState>() -> axum::Router<TState>
where
    TState: Clone + Send + Sync + 'static,
{
    axum::Router::<TState>::new()
        .nest("/api", api_handler::router::<TState>())
        .nest("/function", function_handler::router::<TState>())
}
