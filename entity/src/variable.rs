//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "variable")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub scope_id: Uuid,
    pub name: String,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scope::Entity",
        from = "Column::ScopeId",
        to = "super::scope::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Scope,
}

impl Related<super::scope::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scope.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
