use sea_orm::{prelude::*, Set};

use super::errors::ServiceError;

pub(crate) async fn get_all_services(
    db_pool: &crate::db::DbPool,
) -> Result<Vec<crate::domain::service::Service>, ServiceError> {
    let services: Vec<crate::domain::service::Service> = entity::service::Entity::find()
        .all(db_pool)
        .await?
        .into_iter()
        .map(|service| service.into())
        .collect();
    Ok(services)
}

pub(crate) async fn register_service(
    service_address: &str,
    db_pool: &crate::db::DbPool,
) -> Result<crate::domain::service::Service, ServiceError> {
    // Check if the service already exists by its address
    if let Some(service) = entity::service::Entity::find()
        .filter(entity::service::Column::Address.eq(service_address))
        .one(db_pool)
        .await?
    {
        Ok(service.into())
    } else {
        let service = entity::service::ActiveModel {
            id: Set(Uuid::new_v4()),
            address: Set(service_address.to_string()),
        };
        Ok(service.insert(db_pool).await?.into())
    }
}
