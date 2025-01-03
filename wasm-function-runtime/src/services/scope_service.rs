use std::ops::Deref;

use sea_orm::{prelude::*, Set};

pub(crate) async fn get_all_scopes(
    db_pool: &crate::db::DbPool,
) -> Vec<crate::domain::scope::FunctionScope> {
    // Return all scopes
    let mut scopes: Vec<crate::domain::scope::FunctionScope> = entity::scope::Entity::find()
        .all(db_pool)
        .await
        .expect("Failed to query all scopes")
        .into_iter()
        .map(|scope| scope.into())
        .collect();

    // Sort the scopes by name
    scopes.sort_by(|a, b| a.name.cmp(&b.name));

    // Return the sorted scopes
    scopes
}

pub(crate) async fn get_scope_by_name(
    db_pool: &crate::db::DbPool,
    scope_name: &str,
) -> Option<crate::domain::scope::FunctionScope> {
    entity::scope::Entity::find()
        .filter(entity::scope::Column::Name.eq(scope_name))
        .one(db_pool)
        .await
        .expect("Failed to query scope by name")
        .map(|scope| scope.into())
}

pub(crate) async fn create_or_find_scope(
    db_transaction: &crate::db::DbTransaction,
    scope_name: &str,
) -> crate::domain::scope::FunctionScope {
    match entity::scope::Entity::find()
        .filter(entity::scope::Column::Name.eq(scope_name))
        .one(db_transaction.deref())
        .await
        .unwrap()
    {
        Some(scope) => scope,
        None => {
            let func_scope_active = entity::scope::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(scope_name.to_string()),
            };
            func_scope_active
                .insert(db_transaction.deref())
                .await
                .expect("Failed to insert scope")
        }
    }
    .into()
}

pub(crate) async fn delete_scope(db_pool: &crate::db::DbPool, scope_name: &str) {
    let scope_to_delete = entity::scope::Entity::find()
        .filter(entity::scope::Column::Name.eq(scope_name))
        .one(db_pool)
        .await
        .expect("Failed to query scope by name");

    if let Some(scope) = scope_to_delete {
        scope.delete(db_pool).await.expect("Failed to delete scope");
    }
}
