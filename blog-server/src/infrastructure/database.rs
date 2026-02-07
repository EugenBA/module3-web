use sqlx::PgPool;
use sqlx::migrate::MigrateError;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub(crate) async fn run(pool: &PgPool) -> Result<(), MigrateError> {
    MIGRATOR.run(pool).await
}
