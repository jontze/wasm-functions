pub(crate) async fn extract_http_func(
    cache_registry: &crate::server_state::PluginRegistry,
    scope_name: &str,
    path: &str,
    method: &str,
) -> Option<Vec<u8>> {
    cache_registry
        .get(&cache_key(scope_name, path, method))
        .await
}

fn cache_key(scope_name: &str, path: &str, method: &str) -> String {
    format!("{scope_name}-{path}-{method}")
}

pub(crate) async fn cache_http_func(
    cache_registry: &crate::server_state::PluginRegistry,
    scope_name: &str,
    path: &str,
    method: &str,
    bytes: &[u8],
) {
    cache_registry
        .insert(cache_key(scope_name, path, method), bytes.to_vec())
        .await;
}

pub(crate) async fn invalidate_http_func(
    cache_registry: &crate::server_state::PluginRegistry,
    scope_name: &str,
    path: &str,
    method: &str,
) {
    cache_registry
        .invalidate(&cache_key(scope_name, path, method))
        .await;
}
