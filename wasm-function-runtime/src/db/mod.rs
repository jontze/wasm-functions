use migration::MigratorTrait;

pub(crate) struct DbPool(sea_orm::DatabaseConnection);

impl std::ops::Deref for DbPool {
    type Target = sea_orm::DatabaseConnection;

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
