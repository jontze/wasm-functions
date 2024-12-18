use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::method_routing::get,
};

use crate::{
    bindings_function_http,
    server_state::RuntimeStateRef,
    services::{function_service, wasm_cache_service},
};

pub(crate) fn router() -> axum::Router<RuntimeStateRef> {
    axum::Router::new().route(
        "/*function_path",
        get(handle_get_request).post(handle_post_request),
    )
}

async fn handle_get_request(
    Path(path): Path<String>,
    State(state): State<RuntimeStateRef>,
    Query(query_map): Query<std::collections::HashMap<String, String>>,
    header_map: HeaderMap,
) -> impl IntoResponse {
    let (function, mut function_builder) =
        if let Some(func) = setup_function(state.clone(), &path, "POST").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    // Prepare the request to be passed to the function
    let query_params: Vec<bindings_function_http::QueryParam> = query_map
        .iter()
        .map(|(key, value)| bindings_function_http::QueryParam {
            name: key.clone(),
            value: value.clone(),
        })
        .collect();
    let headers: Vec<bindings_function_http::Header> = header_map
        .iter()
        .map(|(key, value)| bindings_function_http::Header {
            name: key.as_str().to_string(),
            value: value.to_str().unwrap().to_string(),
        })
        .collect();

    let req = bindings_function_http::Request {
        path: format!("/{}", path),
        query_params,
        method: bindings_function_http::Method::Get,
        body: vec![],
        headers,
    };

    // Execute the function
    let function_response = function
        .call_handle_request(&mut function_builder.store, &req)
        .await
        .expect("Failed to call function")
        .expect("Function retunred a failure");

    // Handle the response
    (
        StatusCode::OK,
        [("Content-Type", "text/plain")],
        function_response.body,
    )
        .into_response()
}

async fn handle_post_request(
    Path(path): Path<String>,
    State(state): State<RuntimeStateRef>,
    Query(query_map): Query<std::collections::HashMap<String, String>>,
    header_map: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    let (function, function_builder) =
        if let Some(func) = setup_function(state.clone(), &path, "POST").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    format!("Hello, {}!", path).into_response()
}

async fn setup_function(
    state: RuntimeStateRef,
    path: &str,
    method: &str,
) -> Option<(
    bindings_function_http::FunctionHttp,
    crate::component::http::FunctionHttpBuilder,
)> {
    let registry_read = state.registry.read().await;
    if let Some(precompiled_bytes) =
        wasm_cache_service::extract_http_func(&registry_read, &path, method)
    {
        // If it is, execute the precompiled wasm
        let mut http_function_builder = unsafe {
            crate::component::http::FunctionHttpBuilder::deserialize(
                &state.engine,
                precompiled_bytes,
            )
        };
        drop(registry_read);
        Some((http_function_builder.build().await, http_function_builder))
    } else {
        // If it is not, check the db if there is a function for the route
        let (_, wasm_bytes) = if let Some((http_function, wasm_bytes)) =
            function_service::find_http_func(&state.db, &path, method).await
        {
            (http_function, wasm_bytes)
        } else {
            return None;
        };

        // If there is, extract it from the filesystem and compile it.
        let mut http_function_builder =
            crate::component::http::FunctionHttpBuilder::from_binary(&state.engine, &wasm_bytes);

        // Then save the procompiled wasm to the cache registry to speed up future requests
        {
            let precompiled_bytes = http_function_builder.serialize();
            let mut write_registry = state.registry.write().await;
            wasm_cache_service::cache_http_func(
                &mut write_registry,
                &path,
                method,
                &precompiled_bytes,
            );
        }

        Some((http_function_builder.build().await, http_function_builder))
    }
}
