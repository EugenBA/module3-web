# module3-web
Yandex practicum rust module 3

# 1. blog-server
Сервер на базе actix-web
реализует следуюшие API:
- GET /api/health - статус сервера
- GET /api/posts?limit=0&offset=10 - список постов
- GET /api/posts/id - получение поста по id
- POST /api/posts - созадание поста
- DELETE /api/posts/id - удаление постаё
- PUT /api/posts/id - обновления поста
- POST /api/auth/login - авторизация
- POST /api/auth/register - регистрация
Среврер реализует API по транспорту HTTP и gRPC

### Ручки:
- обновления поста
- создания поста
- удаления поста
Защищены JWT токеном

### Конфигурация:
Конфигурация сервера раализована в .env файле:
DATABASE_URL= строка подключения к базе данных postgresql
HOST= адрес хоста сервера
PORT=порт
JWT_SECRET= JWT секрет
CORS_ORIGINS=настройки CORS, адреса разделенные запятой
GRPC_PORT= порт gRPC

### Запуск и сборка:
cargo run --package blog-server --bin blog-server

# 2. blog-client
библиотека фронтенда для доступа к API сервра blog-server. 
Реализует методы регистрации, входа, создани, обновления, удаления постов. 
Поддерживает транспорт HTTP и gRPC. 
Подерживает сборку под архитектуру wasm32 (без поддержки транспотра gRPC)

### Сборка:
cargo build --package blog-client

# 3. blog-cli
Консольное приложения для доступа к API серврера.
Поддерживает транспорт HTTP и gRPC.

### Поддерживаемые команды:


### Сборка:
cargo build --package blog-cli

# 4. blog-wasm
Библиотека wasm32 реализует фронтенд для доступа к API серверу блогов
Поддерживает протокол HTTP
Реализует API для:
- вход
- регистрацию
- создание поста
- обновление поста
- удаления поста
- получение поста
- получение списка постов.

### Сборка:
wasm-pack build --target web

### Запуск в простом сервере (rust)
simple-http-server --index --nocache --cors --port 8080