use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HttpFunction::Table)
                    .if_not_exists()
                    .col(pk_uuid(HttpFunction::Id).not_null())
                    .col(string(HttpFunction::Name).not_null())
                    .col(string(HttpFunction::Path).not_null())
                    .col(string(HttpFunction::Method).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ScheduledFunction::Table)
                    .if_not_exists()
                    .col(pk_uuid(ScheduledFunction::Id).not_null())
                    .col(string(ScheduledFunction::Name).not_null())
                    .col(string(ScheduledFunction::Cron).not_null())
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
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ScheduledFunction::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum HttpFunction {
    Table,
    Id,
    Name,
    Path,
    Method,
}

#[derive(DeriveIden)]
enum ScheduledFunction {
    Table,
    Id,
    Name,
    Cron,
}
