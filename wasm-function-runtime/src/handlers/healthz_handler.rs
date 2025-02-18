use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::method_routing::get,
};

use crate::{server_state::RuntimeStateRef, services::scope_service};

pub(crate) fn router() -> axum::Router<RuntimeStateRef> {
    axum::Router::new()
        .route("/ready", get(handle_ready_request))
        .route("/live", get(handle_live_request))
}

async fn handle_ready_request(State(state): State<RuntimeStateRef>) -> impl IntoResponse {
    let is_db_available = scope_service::get_all_scopes(&state.db)
        .await
        .is_ok_and(|_| true);

    let is_cache_available = {
        let check_key = "ping";
        let insert_ret = state.cache_backend.insert(check_key, "pong".into()).await;
        let get_ret = state.cache_backend.get(check_key).await;
        let delete_ret = state.cache_backend.invalidate(check_key).await;
        matches!(
            (insert_ret, get_ret, delete_ret),
            (Ok(()), Ok(Some(_)), Ok(()))
        )
    };

    if !is_db_available {
        (StatusCode::SERVICE_UNAVAILABLE, "Database not available")
    } else if !is_cache_available {
        (StatusCode::SERVICE_UNAVAILABLE, "Cache not available")
    } else {
        (StatusCode::OK, "OK")
    }
    .into_response()
}

async fn handle_live_request() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
