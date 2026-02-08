use clap::{Parser, Subcommand};

// Определяем возможные команды
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Регистрация нового пользователя
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Авторизация пользователя
    Login {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    /// Создание новой записи
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    /// Получение записи по ID
    Get {
        #[arg(long)]
        id: i64,
    },
    /// Обновление записи
    Update {
        #[arg(long)]
        id: i64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
    },
    /// Удаление записи
    Delete {
        #[arg(long)]
        id: i64,
    },
    /// Список записей с пагинацией
    List {
        #[arg(long, default_value = "10")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
}

// Определяем структуру для аргументов командной строки
#[derive(Parser, Debug)]
#[command(name = "blog-client")]
#[command(about = "Blog client application", version = "1.0")]
pub(crate) struct Cli {
    /// Адрес сервера (опциональный)
    #[arg(long)]
    pub(crate) server: Option<String>,

    /// Подкоманда для выполнения
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Использовать gRPC транспорт вместо HTTP
    #[arg(long, global = true)]
    pub(crate) grpc: bool,
}
