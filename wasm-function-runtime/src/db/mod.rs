use migration::MigratorTrait;
use sea_orm::TransactionTrait;

pub(crate) struct DbPool(sea_orm::DatabaseConnection);

impl DbPool {
    pub(crate) async fn start_transaction(&self) -> DbTransaction {
        DbTransaction(self.0.begin().await.expect("Failed to start transaction"))
    }
}

impl std::ops::Deref for DbPool {
    type Target = sea_orm::DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
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
