pub(crate) fn extract_http_func<'a>(
    cache_registry: &'a crate::server_state::PluginRegistry,
    path: &str,
    method: &str,
) -> Option<&'a Vec<u8>> {
    cache_registry.get(&cache_key(path, method))
}

fn cache_key(path: &str, method: &str) -> String {
    format!("{}-{}", path, method)
}

pub(crate) fn cache_http_func(
    cache_registry: &mut crate::server_state::PluginRegistry,
    path: &str,
    method: &str,
    bytes: &[u8],
) {
    cache_registry.insert(cache_key(path, method), bytes.to_vec());
}
