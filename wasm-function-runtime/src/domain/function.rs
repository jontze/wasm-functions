use uuid::Uuid;

pub(crate) enum Function {
    Http(HttpFunction),
    Scheduled(ScheduledFunction),
}

pub(crate) trait WasmFunctionTrait {
    fn related_wasm(&self) -> String;
}

pub(crate) struct HttpFunction {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) method: String,
}

impl WasmFunctionTrait for HttpFunction {
    fn related_wasm(&self) -> String {
        format!("http_{}.wasm", self.uuid)
    }
}

impl From<entity::http_function::Model> for HttpFunction {
    fn from(http_function: entity::http_function::Model) -> Self {
        Self {
            uuid: http_function.id,
            name: http_function.name,
            method: http_function.method,
            path: http_function.path,
        }
    }
}

pub(crate) struct ScheduledFunction {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,
    pub(crate) cron: String,
}

impl WasmFunctionTrait for ScheduledFunction {
    fn related_wasm(&self) -> String {
        format!("scheduled_{}.wasm", self.uuid)
    }
}

impl From<entity::scheduled_function::Model> for ScheduledFunction {
    fn from(scheduled_function: entity::scheduled_function::Model) -> Self {
        Self {
            name: scheduled_function.name,
            uuid: scheduled_function.id,
            cron: scheduled_function.cron,
        }
    }
}
