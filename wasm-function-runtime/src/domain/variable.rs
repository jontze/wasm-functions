use uuid::Uuid;

#[allow(unused)]
pub(crate) struct Variable {
    pub uuid: Uuid,
    pub name: String,
    pub value: String,
}

impl From<entity::variable::Model> for Variable {
    fn from(model: entity::variable::Model) -> Self {
        Self {
            uuid: model.id,
            name: model.name,
            value: model.value,
        }
    }
}
