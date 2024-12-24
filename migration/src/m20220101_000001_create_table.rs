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
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
}

#[derive(DeriveIden)]
enum ScheduledFunction {
    Table,
    Id,
    ScopeId,
    Name,
    Cron,
}

const IDX_UNIQUE_SCOPE_HTTP_FUNC_NAME: &str = "idx_unique_scope_http_func_name";

#[derive(DeriveIden)]
enum Scope {
    Table,
    Id,
    Name,
}

const IDX_UNIQUE_SCOPE_SCHEDULED_FUNC_NAME: &str = "idx_unique_scope_scheduled_func_name";
