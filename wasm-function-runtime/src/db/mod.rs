use migration::MigratorTrait;
use sea_orm::TransactionTrait;

pub(crate) struct DbPool(sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl sea_orm::ConnectionTrait for DbPool {
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
    }

    async fn execute(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        self.0.execute(stmt).await
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        self.0.execute_unprepared(sql).await
    }

    async fn query_one(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
        self.0.query_one(stmt).await
    }

    async fn query_all(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
        self.0.query_all(stmt).await
    }
}

impl DbPool {
    pub(crate) async fn start_transaction(&self) -> DbTransaction {
        DbTransaction(self.0.begin().await.expect("Failed to start transaction"))
    }
}

pub(crate) struct DbTransaction(sea_orm::DatabaseTransaction);

impl DbTransaction {
    pub(crate) async fn commit(self) {
        self.0.commit().await.expect("Failed to commit transaction")
    }
}

impl std::ops::Deref for DbTransaction {
    type Target = sea_orm::DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) async fn init_pool(database_url: &str) -> DbPool {
    let pool = sea_orm::Database::connect(database_url)
        .await
        .expect("Failed to connect to database");
    DbPool(pool)
}

pub(crate) async fn run_migrations(pool: &DbPool) {
    migration::Migrator::up(&pool.0, None)
        .await
        .expect("Failed to run migrations");
}
