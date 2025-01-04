use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub(crate) struct FunctionScope {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
}

impl From<entity::scope::Model> for FunctionScope {
    fn from(scope: entity::scope::Model) -> Self {
        Self {
            uuid: scope.id,
            name: scope.name,
        }
    }
}
