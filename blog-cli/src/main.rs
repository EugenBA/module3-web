use std::error::Error;
use clap::Parser;
use blog_client::clients::client::{BlogClient, Transport};
use blog_client::error::BlogClientError;
use crate::cli::{Cli, Commands};


mod cli;


#[tokio::main]
async fn main() ->Result<(), Box<dyn Error>>{
    let cli = Cli::parse();
    // Определяем адрес сервера
    let server_address = if cli.grpc {
        cli.server.unwrap_or_else(|| "http://localhost:50051".to_string())
    } else {
        cli.server.unwrap_or_else(|| "http://127.0.0.1:3000".to_string())
    };

    let transport = if cli.grpc == true {
        Transport::grpc(server_address.clone())
    } else {
        Transport::Http(server_address.clone())
    };
    let timeout = core::time::Duration::from_secs(30);
    let client = BlogClient::new(transport, timeout).await.expect("Error create client");

    println!("Using {} transport", if cli.grpc { "gRPC" } else { "HTTP" });
    println!("Server address: {}", server_address);

    // Загружаем сохраненный токен
    client.load_token().await?;

    // Выполняем команду
    let response = match &cli.command {
        Commands::Register { username, email, password } => {
            let result = client.register(username, email, password).await;
            // Для Register сохраняем токен, если он был получен
            if let Ok(ref result) = result  && let Some(token) = &result.token
            && let Some(user) = &result.user{
                if client.save_token(&token).is_ok() {
                    println!("User: {} register, token saved to .blog_token", user);
                }
            }
            result
        }
        Commands::Login { username, password } => {
            let result = client.login(username, password).await;
            // Для Login сохраняем токен
            if let Ok(ref result) = result && let Some(token) = &result.token {
                if client.save_token(&token).is_ok() && let Some (user) = &result.user{
                    println!("User: {}, login, Token saved to .blog_token", user);
                }
            }
            result
        }
        Commands::Create { title, content } => {
            if let Some(_) = client.get_token().await {
                let result = client.create_post(title, content).await;
                result
            } else {
                Err(BlogClientError::Unauthorized("Token required. Please login first.".to_string()))
            }
        }
        Commands::Get { id } => {
            client.get_post(*id).await
        }
        Commands::Update { id, title, content } => {
            if let Some(_) = client.get_token().await {
                let response = client.get_post(*id).await?;
                if let Some(id) = response.id &&  let Some(mut post_title) = response.title
                 && let Some (mut post_content) = response.content && id == id {
                    if let Some(title) = title {
                        post_title = title.clone();
                    }
                    if let Some(content) = content {
                        post_content = content.clone();
                    }
                    client.update_post(id, &post_title, &post_content).await
                }
                else {
                    Err(BlogClientError::InvalidRequest("Not data post update".to_string()))
                }
            } else {
                Err(BlogClientError::Unauthorized("Token required. Please login first.".to_string()))
            }
        }
        Commands::Delete { id } => {
            if let Some(_) = client.get_token().await {
                let result = client.delete_post(*id).await;
                if result.is_ok(){
                    println!("Post {} deleted", id);
                    client.list_posts(Some(10), Some(0)).await
                }
                else {
                    result
                }
            } else {
                Err(BlogClientError::Unauthorized("Token required. Please login first.".to_string()))
            }
        }
        Commands::List { limit, offset } => {
            client.list_posts(Some(*limit), Some(*offset)).await
        }
    };
    match response {
        Ok(response) => { println!("{}", response.format_output())}
        Err(e) => {println!("Error: {}", e)}
    }
    Ok(())
}

