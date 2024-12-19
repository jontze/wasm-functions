pub(crate) async fn extract_http_func(
    cache_registry: &crate::server_state::PluginRegistry,
    path: &str,
    method: &str,
) -> Option<Vec<u8>> {
    cache_registry.get(&cache_key(path, method)).await
}

fn cache_key(path: &str, method: &str) -> String {
    format!("{}-{}", path, method)
}

pub(crate) async fn cache_http_func(
    cache_registry: &crate::server_state::PluginRegistry,
    path: &str,
    method: &str,
    bytes: &[u8],
) {
    cache_registry
        .insert(cache_key(path, method), bytes.to_vec())
        .await;
}
