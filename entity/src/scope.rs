//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scope")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::http_function::Entity")]
    HttpFunction,
    #[sea_orm(has_many = "super::scheduled_function::Entity")]
    ScheduledFunction,
    #[sea_orm(has_many = "super::secret::Entity")]
    Secret,
    #[sea_orm(has_many = "super::variable::Entity")]
    Variable,
}

impl Related<super::http_function::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HttpFunction.def()
    }
}

impl Related<super::scheduled_function::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduledFunction.def()
    }
}

impl Related<super::secret::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Secret.def()
    }
}

impl Related<super::variable::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Variable.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
