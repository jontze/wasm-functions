use std::ops::Deref;

use sea_orm::{prelude::*, Set};

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
