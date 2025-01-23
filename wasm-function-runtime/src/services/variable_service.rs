use sea_orm::{prelude::*, IntoActiveModel, Set};

use super::{errors::ServiceError, scope_service};
use crate::domain;

pub(crate) async fn find_all_vars(
    db_pool: &crate::db::DbPool,
    scope_name: &str,
) -> Result<Vec<crate::domain::variable::Variable>, ServiceError> {
    if let Some(scope) = scope_service::get_scope_by_name(db_pool, scope_name).await? {
        let mut vars: Vec<domain::variable::Variable> = entity::variable::Entity::find()
            .filter(entity::variable::Column::ScopeId.eq(scope.uuid))
            .all(db_pool)
            .await?
            .into_iter()
            .map(|variable| variable.into())
            .collect();

        // Sort the variables by name
        vars.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(vars)
    } else {
        Ok(vec![])
    }
}

pub(crate) async fn find_var_by_id(
    db_pool: &crate::db::DbPool,
    var_id: &Uuid,
) -> Result<Option<crate::domain::variable::Variable>, ServiceError> {
    Ok(entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await?
        .map(|variable| variable.into()))
}

pub(crate) async fn find_vars_by_scheduled_func_id(
    db_pool: &crate::db::DbPool,
    func_id: &Uuid,
) -> Result<Option<Vec<crate::domain::variable::Variable>>, ServiceError> {
    let func_with_scope = entity::scheduled_function::Entity::find()
        .filter(entity::scheduled_function::Column::Id.eq(*func_id))
        .inner_join(entity::scope::Entity)
        .find_also_related(entity::scope::Entity)
        .one(db_pool)
        .await?;

    if let Some((_, func_scope)) = func_with_scope {
        // It's not possible to have a function without a scope, also we do a inner join before so it should be always present
        let func_scope: entity::scope::Model = func_scope.expect("Function to have a scope");
        let vars: Vec<domain::variable::Variable> = entity::variable::Entity::find()
            .filter(entity::variable::Column::ScopeId.eq(func_scope.id))
            .all(db_pool)
            .await?
            .into_iter()
            .map(|variable| variable.into())
            .collect();

        Ok(Some(vars))
    } else {
        Ok(None)
    }
}

pub(crate) async fn delete_var_by_id(
    db_pool: &crate::db::DbPool,
    var_id: &Uuid,
) -> Result<(), ServiceError> {
    let var_to_delete = entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await?;

    if let Some(var) = var_to_delete {
        var.delete(db_pool).await?;
    }
    Ok(())
}

pub(crate) async fn create_var(
    db_pool: &crate::db::DbPool,
    scope_name: &str,
    name: &str,
    value: &str,
) -> Result<crate::domain::variable::Variable, ServiceError> {
    let scope = scope_service::get_scope_by_name(db_pool, scope_name)
        .await?
        .expect("Scope not found");
    let var_active = entity::variable::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name.to_string()),
        value: Set(value.to_string()),
        scope_id: Set(scope.uuid),
    };

    Ok(var_active.insert(db_pool).await?.into())
}

pub(crate) async fn update_var(
    db_pool: &crate::db::DbPool,
    var_id: &Uuid,
    name: Option<&str>,
    value: Option<&str>,
) -> Result<Option<crate::domain::variable::Variable>, ServiceError> {
    if let Some(var) = entity::variable::Entity::find()
        .filter(entity::variable::Column::Id.eq(*var_id))
        .one(db_pool)
        .await?
    {
        let mut var = var.into_active_model();
        if let Some(name) = name {
            var.name = Set(name.to_string());
        }

        if let Some(value) = value {
            var.value = Set(value.to_string());
        }

        Ok(Some(var.update(db_pool).await?.into()))
    } else {
        Ok(None)
    }
}
