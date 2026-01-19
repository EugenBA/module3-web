mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;

pub mod exchange {
    tonic::include_proto!("blog");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    server::start_server().await?;
    Ok(())
}
