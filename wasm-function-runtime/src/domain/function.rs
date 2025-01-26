use serde::Serialize;
use sha2::Digest;
use uuid::Uuid;

pub(crate) trait WasmFunctionTrait {
    fn uuid(&self) -> Uuid;
    fn name(&self) -> &str;
    fn related_wasm(&self) -> String;
}

#[derive(Serialize)]
pub(crate) enum Function {
    Http(HttpFunction),
    Scheduled(ScheduledFunction),
}

impl Function {
    pub(crate) fn kind(&self) -> &str {
        match self {
            Function::Http(_) => "http",
            Function::Scheduled(_) => "scheduled",
        }
    }

    pub(crate) fn hash(content: &[u8]) -> String {
        let digest_bytes = sha2::Sha256::digest(content);
        hex::encode(digest_bytes)
    }
}

impl WasmFunctionTrait for Function {
    fn uuid(&self) -> Uuid {
        match self {
            Function::Http(http_function) => http_function.uuid(),
            Function::Scheduled(scheduled_function) => scheduled_function.uuid(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Function::Http(http_function) => http_function.name(),
            Function::Scheduled(scheduled_function) => scheduled_function.name(),
        }
    }

    fn related_wasm(&self) -> String {
        match self {
            Function::Http(http_function) => http_function.related_wasm(),
            Function::Scheduled(scheduled_function) => scheduled_function.related_wasm(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct HttpFunction {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) method: String,
    pub(crate) content_hash: String,
}

impl WasmFunctionTrait for HttpFunction {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn related_wasm(&self) -> String {
        format!("http_{}_{}.wasm", self.uuid, self.content_hash)
    }
}

impl From<entity::http_function::Model> for HttpFunction {
    fn from(http_function: entity::http_function::Model) -> Self {
        Self {
            uuid: http_function.id,
            name: http_function.name,
            method: http_function.method,
            path: http_function.path,
            content_hash: http_function.content_hash,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ScheduledFunction {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,
    pub(crate) cron: String,
    pub(crate) content_hash: String,
}

impl WasmFunctionTrait for ScheduledFunction {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn related_wasm(&self) -> String {
        format!("scheduled_{}_{}.wasm", self.uuid, self.content_hash)
    }
}

impl From<entity::scheduled_function::Model> for ScheduledFunction {
    fn from(scheduled_function: entity::scheduled_function::Model) -> Self {
        Self {
            name: scheduled_function.name,
            uuid: scheduled_function.id,
            cron: scheduled_function.cron,
            content_hash: scheduled_function.content_hash,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_function_hash() {
        let content = b"test";
        let hash = Function::hash(content);
        assert_eq!(hash.len(), 64);
    }
}
