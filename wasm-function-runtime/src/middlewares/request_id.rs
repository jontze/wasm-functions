#[derive(Default, Clone)]
pub(crate) struct RequstIdGenerator {
    counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl tower_http::request_id::MakeRequestId for RequstIdGenerator {
    fn make_request_id<B>(
        &mut self,
        _: &http::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        let request_id = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            .to_string()
            .parse()
            .unwrap();

        Some(tower_http::request_id::RequestId::new(request_id))
    }
}
