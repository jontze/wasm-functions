use std::time;

use sea_orm::Set;
use uuid::Uuid;

use super::errors::ServiceError;
use crate::{db::DbPool, domain};

pub(crate) async fn join_new(
    db: &DbPool,
    addr: &str,
) -> Result<domain::service::Service, ServiceError> {
    let service = entity::service::ActiveModel {
        id: Set(Uuid::new_v4()),
        address: Set(addr.to_string()),
        joined_at: Set(time::now()),
        last_heartbeat: Set(time::PrimitiveDateTime),
        status: Set("joining".to_string()),
    };
    let service = service.insert(db).await?;
    todo!()
}
