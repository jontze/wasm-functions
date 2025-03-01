use uuid::Uuid;

pub(crate) struct Service {
    pub uuid: Uuid,
    pub address: String,
}

impl From<entity::service::Model> for Service {
    fn from(service: entity::service::Model) -> Self {
        Self {
            uuid: service.id,
            address: service.address,
        }
    }
}
