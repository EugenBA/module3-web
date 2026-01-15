mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    server::start_server().await?;
    Ok(())
}
