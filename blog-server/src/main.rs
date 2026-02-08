//! Сервер блога
//!
//! Реализует сервер с танспортом HTTP и gRPC для блогинга

#![warn(missing_docs)]
mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;
mod server;

#[allow(missing_docs)]
pub mod blog {
    tonic::include_proto!("blog");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    server::start_server().await.expect("Error start server");
    Ok(())
}
