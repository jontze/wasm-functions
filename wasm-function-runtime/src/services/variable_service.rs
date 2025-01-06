use sea_orm::{prelude::*, IntoActiveModel, Set};

use super::scope_service;
use crate::domain;

pub(crate) async fn find_all_vars(
    db_pool: &crate::db::DbPool,
    scope_name: &str,
) -> Vec<crate::domain::variable::Variable> {
    if let Some(scope) = scope_service::get_scope_by_name(db_pool, scope_name).await {
        let mut vars: Vec<domain::variable::Variable> = entity::variable::Entity::find()
            .filter(entity::variable::Column::ScopeId.eq(scope.uuid))
            .all(db_pool)
            .await
            .expect("Failed to query all variables")
            .into_iter()
            .map(|variable| variable.into())
            .collect();

        // Sort the variables by name
        vars.sort_by(|a, b| a.name.cmp(&b.name));

        vars
    } else {
        vec![]
    }
}

pub(crate) async fn find_var_by_id(
    db_pool: &crate::db::DbPool,
    var_id: &Uuid,
) -> Option<crate::domain::variable::Variable> {
    entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await
        .expect("Failed to query variable by id")
        .map(|variable| variable.into())
}

pub(crate) async fn delete_var_by_id(db_pool: &crate::db::DbPool, var_id: &Uuid) {
    let var_to_delete = entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await
        .expect("Failed to query variable by id");

    if let Some(var) = var_to_delete {
        var.delete(db_pool)
            .await
            .expect("Failed to delete variable by id");
    }
}

pub(crate) async fn create_var(
    db_pool: &crate::db::DbPool,
    scope_name: &str,
    name: &str,
    value: &str,
) -> crate::domain::variable::Variable {
    let scope = scope_service::get_scope_by_name(db_pool, scope_name)
        .await
        .expect("Scope not found");
    let var_active = entity::variable::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name.to_string()),
        value: Set(value.to_string()),
        scope_id: Set(scope.uuid),
    };

    var_active
        .insert(db_pool)
        .await
        .expect("Failed to insert variable")
        .into()
}

pub(crate) async fn update_var(
    db_pool: &crate::db::DbPool,
    var_id: &Uuid,
    name: Option<&str>,
    value: Option<&str>,
) -> Option<crate::domain::variable::Variable> {
    if let Some(var) = entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await
        .expect("Failed to query variable by id")
    {
        let mut var = var.into_active_model();
        if let Some(name) = name {
            var.name = Set(name.to_string());
        }

        if let Some(value) = value {
            var.value = Set(value.to_string());
        }

        Some(
            var.update(db_pool)
                .await
                .expect("Failed to update variable")
                .into(),
        )
    } else {
        None
    }
}
