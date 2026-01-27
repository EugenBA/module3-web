use clap::Parser;
use crate::cli::{Cli, Commands};

mod cli;
mod error;

fn main() {
    let cli = Cli::parse();
    // Определяем адрес сервера
    let server_address = if cli.grpc {
        cli.server.unwrap_or_else(|| "localhost:50051".to_string())
    } else {
        cli.server.unwrap_or_else(|| "http://localhost:8080".to_string())
    };

    // Создаем клиент с выбранным транспортом
    let client: Box<dyn BlogClient> = if cli.grpc {
        Box::new(GrpcClient::new(&server_address))
    } else {
        Box::new(HttpClient::new(&server_address))
    };

    println!("Using {} transport", if cli.grpc { "gRPC" } else { "HTTP" });
    println!("Server address: {}", server_address);

    // Загружаем сохраненный токен
    let token = load_token();
    if let Some(t) = &token {
        println!("Loaded token from file: {}...", &t[..10.min(t.len())]);
    }

    // Выполняем команду
    let result = match &cli.command {
        Commands::Register { username, email, password } => {
            let result = client.register(username, email, password);
            // Для Register сохраняем токен, если он был получен
            if let Ok(ref token) = result {
                if save_token(token).is_ok() {
                    println!("Token saved to .blog_token");
                }
            }
            result
        }
        Commands::Login { username, password } => {
            let result = client.login(username, password);
            // Для Login сохраняем токен
            if let Ok(ref token) = result {
                if save_token(token).is_ok() {
                    println!("Token saved to .blog_token");
                }
            }
            result
        }
        Commands::Create { title, content } => {
            if let Some(t) = &token {
                client.create_post(title, content, t)
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::Get { id } => {
            client.get_post(id, token.as_deref())
        }
        Commands::Update { id, title, content } => {
            if let Some(t) = &token {
                client.update_post(id, title.as_deref(), content.as_deref(), t)
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::Delete { id } => {
            if let Some(t) = &token {
                client.delete_post(id, t)
            } else {
                Err("Token required. Please login first.".to_string())
            }
        }
        Commands::List { limit, offset } => {
            client.list_posts(*limit, *offset, token.as_deref())
        }
    };

    // Выводим результат
    match result {
        Ok(response) => println!("Success: {}", response),
        Err(error) => eprintln!("Error: {}", error),
    }
}
    
}
