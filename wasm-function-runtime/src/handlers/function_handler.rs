use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::method_routing::get,
};

use crate::{
    bindings_function_http,
    domain::function::WasmFunctionTrait,
    server_state::RuntimeStateRef,
    services::{function_service, variable_service},
};

pub(crate) fn router() -> axum::Router<RuntimeStateRef> {
    axum::Router::new().route(
        "/{scope}/{*function_path}",
        get(handle_get_request).post(handle_post_request),
    )
}

#[derive(Debug, serde::Deserialize)]
struct FunctionParams {
    scope: String,
    function_path: String,
}

async fn handle_get_request(
    Path(path): Path<FunctionParams>,
    State(state): State<RuntimeStateRef>,
    Query(query_map): Query<std::collections::HashMap<String, String>>,
    header_map: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    // Bootstrap the function
    let (function, mut function_store) =
        if let Some(func) = bootstrap_function(state.clone(), &path, "GET").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    // Prepare the request to be passed to the function
    let req = bindings_function_http::Request {
        path: format!("/{}", path.function_path),
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
        .call_handle_request(&mut function_store, &req)
        .await
        .expect("Failed to call function")
        .expect("Function retunred a failure");

    // Return the response
    function_response.into_response()
}

async fn handle_post_request(
    Path(path): Path<FunctionParams>,
    State(state): State<RuntimeStateRef>,
    Query(query_map): Query<std::collections::HashMap<String, String>>,
    header_map: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    // Bootstrap the function
    let (function, mut function_store) =
        if let Some(func) = bootstrap_function(state.clone(), &path, "POST").await {
            func
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

    // Prepare the request to be passed to the function
    let req = bindings_function_http::Request {
        path: format!("/{}", path.function_path),
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
        .call_handle_request(&mut function_store, &req)
        .await
        .expect("Failed to call function")
        .expect("Function retunred a failure");

    // Return the response
    function_response.into_response()
}

async fn bootstrap_function(
    state: RuntimeStateRef,
    path: &FunctionParams,
    method: &str,
) -> Option<(
    bindings_function_http::FunctionHttp,
    wasmtime::Store<crate::component::ComponentState>,
)> {
    let function_vars = variable_service::find_all_vars(&state.db, &path.scope)
        .await
        .expect("Failed to find variables");

    // Extract the target funtion from the database
    let http_function_details = function_service::find_http_func_by_scope_and_req(
        &state.db,
        &path.scope,
        &path.function_path,
        method,
    )
    .await
    .expect("Failed to find function");

    let http_function_details = match http_function_details {
        Some(details) => details,
        None => return None,
    };

    // Try to get previously compiled function from the cache
    if let Some(cached_function_bytes) = state
        .function_cache
        .get(&http_function_details.related_wasm())
        .await
    {
        // Deserialize the function from the cache
        let http_function_builder = unsafe {
            crate::component::http::FunctionHttpBuilder::deserialize(
                &state.engine,
                &cached_function_bytes,
            )
            .with_variables(&function_vars)
        };

        // Build the function
        Some(http_function_builder.build().await)
    } else {
        // Extract the function from the storage backend
        let function_bytes = state
            .storage_backend
            .extract_file_bytes(&http_function_details.related_wasm())
            .await
            .expect("Failed to get function");

        // Compile the function from the bytes
        let http_function_builder = crate::component::http::FunctionHttpBuilder::from_binary(
            &state.engine,
            &function_bytes,
        )
        .with_variables(&function_vars);

        // Cache the compiled function
        state
            .function_cache
            .insert(
                &http_function_details.related_wasm(),
                http_function_builder.serialize(),
            )
            .await;

        // Build the function
        Some(http_function_builder.build().await)
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
