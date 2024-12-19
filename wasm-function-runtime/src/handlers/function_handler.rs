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
    body: Body,
) -> impl IntoResponse {
    // Bootstrap the function
    let (function, mut function_builder) =
        if let Some(func) = bootstrap_function(state.clone(), &path, "GET").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    // Prepare the request to be passed to the function
    let req = bindings_function_http::Request {
        path: format!("/{}", path),
        query_params: collect_query_params(query_map),
        headers: collect_headers(header_map),
        method: bindings_function_http::Method::Get,
        body: axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("Failed to read body")
            .to_vec(),
    };

    // Execute the function
    let function_response = function
        .call_handle_request(&mut function_builder.store, &req)
        .await
        .expect("Failed to call function")
        .expect("Function retunred a failure");

    // Return the response
    function_response.into_response()
}

async fn handle_post_request(
    Path(path): Path<String>,
    State(state): State<RuntimeStateRef>,
    Query(query_map): Query<std::collections::HashMap<String, String>>,
    header_map: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    // Bootstrap the function
    let (function, mut function_builder) =
        if let Some(func) = bootstrap_function(state.clone(), &path, "POST").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    // Prepare the request to be passed to the function
    let req = bindings_function_http::Request {
        path: format!("/{}", path),
        query_params: collect_query_params(query_map),
        headers: collect_headers(header_map),
        method: bindings_function_http::Method::Post,
        body: axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("Failed to read body")
            .to_vec(),
    };

    // Execute the function
    let function_response = function
        .call_handle_request(&mut function_builder.store, &req)
        .await
        .expect("Failed to call function")
        .expect("Function retunred a failure");

    // Return the response
    function_response.into_response()
}

async fn bootstrap_function(
    state: RuntimeStateRef,
    path: &str,
    method: &str,
) -> Option<(
    bindings_function_http::FunctionHttp,
    crate::component::http::FunctionHttpBuilder,
)> {
    if let Some(precompiled_bytes) =
        wasm_cache_service::extract_http_func(&state.registry, &path, method).await
    {
        // If it is, execute the precompiled wasm
        let mut http_function_builder = unsafe {
            crate::component::http::FunctionHttpBuilder::deserialize(
                &state.engine,
                &precompiled_bytes,
            )
        };
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
        let precompiled_bytes = http_function_builder.serialize();
        wasm_cache_service::cache_http_func(&state.registry, &path, method, &precompiled_bytes)
            .await;

        Some((http_function_builder.build().await, http_function_builder))
    }
}

fn collect_query_params(
    query_map: std::collections::HashMap<String, String>,
) -> Vec<bindings_function_http::QueryParam> {
    query_map
        .iter()
        .map(|(key, value)| bindings_function_http::QueryParam {
            name: key.clone(),
            value: value.clone(),
        })
        .collect()
}

fn collect_headers(header_map: HeaderMap) -> Vec<bindings_function_http::Header> {
    header_map
        .iter()
        .map(|(key, value)| bindings_function_http::Header {
            name: key.as_str().to_string(),
            value: value.to_str().unwrap().to_string(),
        })
        .collect()
}

impl IntoResponse for bindings_function_http::Response {
    fn into_response(self) -> axum::http::Response<Body> {
        let mut response = axum::http::Response::new(self.body.into());
        for header in self.headers {
            response.headers_mut().insert(
                http::HeaderName::from_bytes(header.name.as_bytes()).expect("Invalid header name"),
                http::HeaderValue::from_str(&header.value).expect("Invalid header value"),
            );
        }
        *response.status_mut() =
            http::StatusCode::from_u16(self.status_code).expect("Invalid status code");
        response
    }
}
