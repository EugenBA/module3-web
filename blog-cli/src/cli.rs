use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

// Определяем возможные команды
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Регистрация нового пользователя
    Register {
        username: String,
        email: String,
        password: String,
    },
    /// Авторизация пользователя
    Login {
        username: String,
        password: String,
    },
    /// Создание новой записи
    Create {
        title: String,
        content: String,
    },
    /// Получение записи по ID
    Get {
        id: i64,
    },
    /// Обновление записи
    Update {
        id: i64,
        title: Option<String>,
        content: Option<String>,
    },
    /// Удаление записи
    Delete {
        id: i64,
    },
    /// Список записей с пагинацией
    List {
        #[arg(short, long, default_value = "10")]
        limit: i64,
        #[arg(short, long, default_value = "0")]
        offset: i64,
    },
}

// Определяем структуру для аргументов командной строки
#[derive(Parser, Debug)]
#[command(name = "blog-client")]
#[command(about = "Blog client application", version = "1.0")]
pub(crate) struct Cli {
    /// Использовать gRPC транспорт вместо HTTP
    #[arg(long)]
    pub(crate) grpc: bool,

    /// Адрес сервера (опциональный)
    #[arg(long)]
    pub(crate) server: Option<String>,

    /// Подкоманда для выполнения
    #[command(subcommand)]
    command: Commands,
}

