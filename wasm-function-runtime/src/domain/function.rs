use uuid::Uuid;

pub(crate) enum Function {
    Http(HttpFunction),
    Scheduled(ScheduledFunction),
}

pub(crate) trait WasmFunction {
    fn related_wasm(&self) -> String;
}

pub(crate) struct HttpFunction {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) method: String,
}

impl WasmFunction for HttpFunction {
    fn related_wasm(&self) -> String {
        format!("http_{}_{}_{}.wasm", self.name, self.method, self.uuid)
    }
}

pub(crate) struct ScheduledFunction {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,
    pub(crate) cron: String,
}

impl WasmFunction for ScheduledFunction {
    fn related_wasm(&self) -> String {
        format!("scheduled_{}_{}.wasm", self.name, self.uuid)
    }
}
