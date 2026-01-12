mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;

use infrastructure::{config::Config, database};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let cfg = Config::from_env().expect("invalid config");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    // миграции
    database::run(&pool).await.expect("migrations failed");
    Ok(())
}
