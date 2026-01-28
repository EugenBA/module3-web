use clap::Parser;
use blog_client::clients::client::{BlogClient, Transport};
use blog_client::error::BlogClientError;
use crate::cli::{Cli, Commands};

mod cli;
mod error;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // Определяем адрес сервера
    let server_address = if cli.grpc {
        cli.server.unwrap_or_else(|| "localhost:50051".to_string())
    } else {
        cli.server.unwrap_or_else(|| "http://localhost:8080".to_string())
    };

    let transport = if cli.grpc == true {
        Transport::grpc(cli.server)
    } else {
        Transport::Http(cli.server)
    };
    let client = BlogClient::new(transport).await.expect("Error create client");

    println!("Using {} transport", if cli.grpc { "gRPC" } else { "HTTP" });
    println!("Server address: {}", server_address);

    // Загружаем сохраненный токен
    let token = client.load_token();

    // Выполняем команду
    let result = match &cli.command {
        Commands::Register { username, email, password } => {
            let result = client.register(username, email, password).await;
            // Для Register сохраняем токен, если он был получен
            if let Ok(ref token) = result {
                if client.save_token(&token.token).is_ok() {
                    println!("Token saved to .blog_token");
                }
            }
            result
        }
        Commands::Login { username, password } => {
            let result = client.login(username, password).await;
            // Для Login сохраняем токен
            if let Ok(ref token) = result {
                if client.save_token(&token.token).is_ok() {
                    println!("Token saved to .blog_token");
                }
            }
            result
        }
        Commands::Create { title, content } => {
            if let Some(_) = client.get_token().await {
                client.create_post(title, content).await
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::Get { id } => {
            client.get_post(*id).await
        }
        Commands::Update { id, title, content } => {
            if let Some(t) = client.get_token().await {
                client.update_post(*id, title, content).await
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::Delete { id } => {
            if let Some(t) = client.get_token().await {
                client.delete_post(*id).await
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::List { limit, offset } => {
            client.list_posts(*limit, *offset, token.as_deref()).await
        }
    };

    // Выводим результат
    match result {
        Ok(response) => println!("Success: {}", response),
        Err(error) => eprintln!("Error: {}", error),
    }
}

