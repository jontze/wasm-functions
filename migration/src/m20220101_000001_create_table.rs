use sea_orm::{EnumIter, Iterable};

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ***************************
        // **** Start Scope Table ****
        // ***************************
        manager
            .create_table(
                Table::create()
                    .table(Scope::Table)
                    .if_not_exists()
                    .col(pk_uuid(Scope::Id).not_null().unique_key())
                    .col(string(Scope::Name).not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        // ***************************
        // **** Start Variables ******
        // ***************************
        let mut variable_scope_id_fk = ForeignKey::create()
            .from(Variable::Table, Variable::ScopeId)
            .to(Scope::Table, Scope::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Variable::Table)
                    .if_not_exists()
                    .col(pk_uuid(Variable::Id).not_null().unique_key())
                    .col(uuid(Variable::ScopeId).not_null())
                    .col(string(Variable::Name).not_null())
                    .col(string(Variable::Value).not_null())
                    .foreign_key(&mut variable_scope_id_fk)
                    .to_owned(),
            )
            .await?;

        // ***************************
        // **** Start Secrets ********
        // ***************************
        let mut secret_scope_id_fk = ForeignKey::create()
            .from(Secret::Table, Secret::ScopeId)
            .to(Scope::Table, Scope::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Secret::Table)
                    .if_not_exists()
                    .col(pk_uuid(Secret::Id).not_null().unique_key())
                    .col(uuid(Secret::ScopeId).not_null())
                    .col(string(Secret::Name).not_null())
                    .col(string(Secret::Value).not_null())
                    .foreign_key(&mut secret_scope_id_fk)
                    .to_owned(),
            )
            .await?;

        // ***************************
        // **** Start HTTP Func Table
        // ***************************
        let mut http_func_scope_id_fk = ForeignKey::create()
            .from(HttpFunction::Table, HttpFunction::ScopeId)
            .to(Scope::Table, Scope::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(HttpFunction::Table)
                    .if_not_exists()
                    .col(pk_uuid(HttpFunction::Id).not_null().unique_key())
                    .col(uuid(HttpFunction::ScopeId).not_null())
                    .col(string(HttpFunction::Name).not_null())
                    .col(string(HttpFunction::Path).not_null())
                    .col(string(HttpFunction::Method).not_null())
                    .col(boolean(HttpFunction::IsPublic).not_null())
                    .col(string(HttpFunction::ContentHash).not_null())
                    .foreign_key(&mut http_func_scope_id_fk)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_UNIQUE_SCOPE_HTTP_FUNC_NAME)
                    .if_not_exists()
                    .table(HttpFunction::Table)
                    .col(HttpFunction::ScopeId)
                    .col(HttpFunction::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // ***************************
        // **** Start Scheduled Func Table
        // ***************************
        let mut scheduled_func_scope_id_fk = ForeignKey::create()
            .from(ScheduledFunction::Table, ScheduledFunction::ScopeId)
            .to(Scope::Table, Scope::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ScheduledFunction::Table)
                    .if_not_exists()
                    .col(pk_uuid(ScheduledFunction::Id).not_null().unique_key())
                    .col(uuid(ScheduledFunction::ScopeId).not_null())
                    .col(string(ScheduledFunction::Name).not_null())
                    .col(string(ScheduledFunction::Cron).not_null())
                    .col(string(ScheduledFunction::ContentHash).not_null())
                    .foreign_key(&mut scheduled_func_scope_id_fk)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_UNIQUE_SCOPE_SCHEDULED_FUNC_NAME)
                    .if_not_exists()
                    .table(ScheduledFunction::Table)
                    .col(ScheduledFunction::ScopeId)
                    .col(ScheduledFunction::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // ***************************
        // **** Start Service Registry Table
        // ***************************
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Service::Table)
                    .col(pk_uuid(Service::Id).not_null().unique_key())
                    .col(string(Service::Address).not_null())
                    .col(
                        enumeration(Service::Status, Alias::new("status"), ServiceStatus::iter())
                            .not_null(),
                    )
                    .col(date_time(Service::LastHeartbeat).not_null())
                    .col(date_time(Service::JoinedAt).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Variable::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Secret::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(HttpFunction::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name(IDX_UNIQUE_SCOPE_HTTP_FUNC_NAME)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ScheduledFunction::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name(IDX_UNIQUE_SCOPE_SCHEDULED_FUNC_NAME)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Scope::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Service::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum HttpFunction {
    Table,
    Id,
    ScopeId,
    Name,
    Path,
    Method,
    IsPublic,
    ContentHash,
}

#[derive(DeriveIden)]
enum ScheduledFunction {
    Table,
    Id,
    ScopeId,
    Name,
    Cron,
    ContentHash,
}

const IDX_UNIQUE_SCOPE_HTTP_FUNC_NAME: &str = "idx_unique_scope_http_func_name";

#[derive(DeriveIden)]
enum Scope {
    Table,
    Id,
    Name,
}

const IDX_UNIQUE_SCOPE_SCHEDULED_FUNC_NAME: &str = "idx_unique_scope_scheduled_func_name";

#[derive(DeriveIden)]
enum Variable {
    Table,
    Id,
    ScopeId,
    Name,
    Value,
}

#[derive(DeriveIden)]
enum Secret {
    Table,
    Id,
    ScopeId,
    Name,
    Value,
}

#[derive(DeriveIden)]
enum Service {
    Table,
    Id,
    Address,
    Status,
    LastHeartbeat,
    JoinedAt,
}

#[derive(Iden, EnumIter)]
enum ServiceStatus {
    #[iden = "up"]
    Up,
    #[iden = "down"]
    Down,
    #[iden = "joining"]
    Joining,
    #[iden = "leaving"]
    Leaving,
    #[iden = "unknown"]
    Unknown,
}
