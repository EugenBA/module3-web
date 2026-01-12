# Задание
Вам предстоит написать четыре крейта в одном Cargo workspace:

    веб-сервер с HTTP и gRPC API,
    клиентскую библиотеку,
    CLI-клиент и WASM-фронтенд.

Задача по факту является реальной разработкой. Этот проект вы при желании можете повесить на домен и иметь свой личный блог с регистрацией юзеров, защитой JWT и безопасным хранением в БД.
Структура проекта должна быть такой:
```
blog-project/
├── Cargo.toml              # Workspace конфигурация
├── README.md               # Описание проекта, инструкции по запуску
├── blog-server/            # Крейт 1: Веб-сервер блога
│   ├── Cargo.toml
│   ├── build.rs
│   ├── proto/
│   │   └── blog.proto
│   └── src/
│       ├── main.rs
│       ├── server.rs
│       ├── handlers.rs
│       ├── domain/
│       ├── application/
│       ├── data/
│       └── infrastructure/
├── blog-client/            # Крейт 2: Библиотека клиента
│   ├── Cargo.toml
│   ├── build.rs
│   ├── proto/
│   │   └── blog.proto      # Копия из blog-server
│   └── src/
│       ├── lib.rs
│       ├── http_client.rs
│       └── grpc_client.rs
├── blog-cli/               # Крейт 3: CLI-клиент
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── blog-wasm/              # Крейт 4: WASM фронтенд
├── Cargo.toml
└── src/
└── lib.rs
```

# Пошаговое выполнение
# Шаг 1. Создание workspace и базовой структуры
Первым делом вам нужно создать новый репозиторий на Гитхабе. 
Инициализируйте проект — подключите его к удалённому репозиторию. Далее настройте воркспейс для крейтов в основном Cargo.toml. 
Настройте в крейте WASM crate-type в локальном Cargo.toml.  Проработайте .gitignore.  Создайте все необходимые папки из структуры проекта.

<details>
<summary><b>Подсказки к шагу 1</b></summary>

    Создайте новый репозиторий на GitHub и клонируйте его локально. Или же Cargo init.
    Создайте Cargo workspace в корне проекта:
        Создайте Cargo.toml с конфигурацией workspace.
        Укажите четыре члена workspace: blog-server, blog-client, blog-cli, blog-wasm.
        Настройте общие зависимости в [workspace.dependencies] для переиспользования между крейтами.
    Создайте четыре крейта:
        blog-client — библиотека (cargo new --lib),
        blog-server — бинарный крейт (cargo new),
        blog-cli — бинарный крейт (cargo new),
        blog-wasm — библиотека для WASM (cargo new --lib).
    Настройте blog-wasm для компиляции в WebAssembly:
        В blog-wasm/Cargo.toml укажите crate-type = \["cdylib"\] в секции [lib].
        Если используемый фреймворк (например, wasm-pack, cargo leptos, trunk) поддерживает создание шаблонов, можно сгенерировать стартовый package и затем адаптировать его под структуру задания.
    Создайте необходимые папки:
        blog-server/migrations/ — для SQL-миграций.
        blog-server/proto/ — для Protocol Buffers схемы.
        blog-client/proto/ — для копии proto-схемы.
    Настройте .gitignore: добавьте target/, .env, .blog_token, Cargo.lock и другие артефакты сборки.

</details>

# Шаг 2. Определение protobuf-схемы
В файле .proto сервера, работающего на gRPC:

    Определите методы аутентификации (Логин+регистрация) и операции с постами (CreatePost, GetPost, UpdatePost, DeletePost, ListPosts).
    Определите message типы.
    Перенесите схему блога сервера в крейт клиента.
    В обоих крейтах должен быть настроен Build.rs (cargo:rerun-if-changed используйте для автоматической пересборки при изменении схемы).


<details>
<summary><b>Подсказки к шагу 2</b></summary>
Создайте файл blog-server/proto/blog.proto с определением gRPC-сервиса:

    Определите service BlogService с методами для аутентификации (Register, Login) и CRUD-операций с постами (CreatePost, GetPost, UpdatePost, DeletePost, ListPosts).
    Определите все необходимые message типы: RegisterRequest, LoginRequest, AuthResponse, User, CreatePostRequest, Post, PostResponse и другие.
    Используйте синтаксис proto3.

Скопируйте blog.proto в blog-client/proto/blog.proto — клиент должен использовать ту же схему для генерации кода.
Настройте build.rs в обоих крейтах (blog-server и blog-client):

    Используйте tonic-build для генерации Rust кода из proto-файлов.
    В сервере включите генерацию и сервера, и клиента.
    В клиенте включите только генерацию клиента.
    Добавьте println!("cargo:rerun-if-changed=proto/blog.proto") для автоматической пересборки при изменении proto-файла.
</details>

# Шаг 3. Реализация веб-сервера (blog-server)
Время реализовать приложение, которое будет выполнять роль сервера блога. Тут вы будете держать HTTP API и gRPC API. По сути, это и есть ваш бэкенд. Сюда будут входить миграции, работа с БД через sqlx, Middleware, и безопасность.
Требования:

    HTTP API на порте 3000 (actix-web) — можно изменить на 8080, если нужно.
    gRPC API на порте 50051 (tonic).
    Хранение данных в PostgreSQL через sqlx.
    Миграции для создания таблиц users и posts (используйте BIGSERIAL/BIGINT для ID, чтобы соответствовать Rust i64).
    JWT-аутентификация для защиты операций создания, редактирования и удаления постов.
    Обработка ошибок через кастомные типы ошибок (thiserror).
    Логирование через tracing.
    CORS настроен для работы с WASM-фронтендом.

Настройка зависимостей. Добавьте в blog-server/Cargo.toml необходимые зависимости:

    HTTP-сервер: actix-web, actix-web-httpauth, actix-cors.
    gRPC: tonic, tonic-build (в build-dependencies), prost, prost-types.
    База данных: sqlx с features для PostgreSQL, dotenvy для переменных окружения.
    JWT и безопасность: jsonwebtoken, argon2 для хеширования паролей.
    Утилиты: serde, serde_json, chrono, anyhow, thiserror, tracing, tracing-subscriber.
    Используйте workspace-зависимости, где возможно.

Структура проекта. Используйте clean architecture из предыдущих уроков:
```
blog-server/
├── Cargo.toml
├── build.rs
├── migrations/
│   ├── 20250101000001_create_users.sql
│   └── 20250101000002_create_posts.sql
├── proto/
│   └── blog.proto
└── src/
├── main.rs
├── domain/
│   ├── mod.rs
│   ├── post.rs          # Доменная модель Post
│   ├── user.rs          # Доменная модель User
│   └── error.rs         # Доменные ошибки
├── application/
│   ├── mod.rs
│   ├── auth_service.rs  # Сервис аутентификации
│   └── blog_service.rs  # Бизнес-логика блога
├── data/
│   ├── mod.rs
│   ├── user_repository.rs  # Репозиторий пользователей
│   └── post_repository.rs  # Репозиторий постов
├── infrastructure/
│   ├── mod.rs
│   ├── database.rs      # Подключение к БД
│   ├── jwt.rs          # Работа с JWT токенами
│   └── logging.rs       # Настройка tracing
└── presentation/
├── mod.rs
├── middleware.rs    # JWT middleware для actix-web
├── http_handlers.rs  # HTTP handlers для actix-web
└── grpc_service.rs  # gRPC сервис для tonic
```
Доменные модели. Создайте доменные модели в src/domain/:

    В user.rs определите структуры:
        User — пользователь с полями: id, username, email, password_hash, created_at.
        Структуры для запросов: регистрация (username, email, password) и вход (username, password).
        Используйте serde для сериализации/десериализации.
    В post.rs определите структуры:
        Post — пост с полями: id, title, content, author_id, created_at, updated_at.
        Структуры для запросов: создание (title, content) и обновление (title, content).
        Реализуйте метод new для создания нового поста.
    В error.rs определите кастомные типы ошибок через thiserror:
        UserNotFound, UserAlreadyExists, InvalidCredentials.
        PostNotFound, Forbidden (для случаев, когда пользователь пытается изменить чужой пост).
        Другие необходимые ошибки.

Миграции:

    Создайте файл blog-server/migrations/20250101000001_create_users.sql:
        Создайте таблицу users с полями: id (BIGSERIAL PRIMARY KEY), username (VARCHAR, UNIQUE), email (VARCHAR, UNIQUE), password_hash (VARCHAR), created_at (TIMESTAMP WITH TIME ZONE).
        Добавьте индексы на username и email для быстрого поиска.
    Создайте файл blog-server/migrations/20250101000002_create_posts.sql:
        Создайте таблицу posts с полями: id (BIGSERIAL PRIMARY KEY), title (VARCHAR), content (TEXT), author_id (BIGINT, FOREIGN KEY на users.id), created_at, updated_at (TIMESTAMP WITH TIME ZONE).
        Добавьте внешний ключ на users(id) с ON DELETE CASCADE.
        Добавьте индексы на created_at и author_id.
    Настройте автоматическое применение миграций:
        В src/infrastructure/database.rs создайте функцию для подключения к БД (connection pool).
        Создайте функцию для применения миграций через sqlx::migrate!() макрос.
        Вызовите эти функции в main.rs при старте сервера.

Настройка build.rs для gRPC. Создайте blog-server/build.rs:

    Используйте tonic_build::configure() для настройки генерации кода.
    Включите генерацию сервера и клиента (.build_server(true).build_client(true)).
    Укажите путь к proto-файлу и директорию с proto-файлами.
    Добавьте println!("cargo:rerun-if-changed=proto/blog.proto") для автоматической пересборки при изменении proto-файла.

JWT-аутентификация. Реализуйте модуль для работы с JWT-токенами в src/infrastructure/jwt.rs:

    Определите структуру Claims с полями: user_id, username, exp (время истечения).
    Создайте структуру JwtService с полями для encoding и decoding ключей.
    Реализуйте методы:
        new(secret: &str) — создание сервиса из секретного ключа.
        generate_token(user_id, username) — генерация JWT-токена с временем жизни 24 часа.
        verify_token(token) — проверка и декодирование токена.
    Используйте библиотеку jsonwebtoken для работы с токенами.

Настройка подключения к БД. Создайте src/infrastructure/database.rs:

    Реализуйте функцию create_pool(database_url):
        Создайте connection pool через PgPoolOptions.
        Настройте максимальное количество соединений (например, 5).
        Верните PgPool.
    Реализуйте функцию run_migrations(pool):
        Используйте макрос sqlx::migrate!() для указания пути к миграциям.
        Примените миграции к пулу соединений.
        Верните результат операции.

HTTP API endpoints. Реализуйте следующие endpoints в presentation/http_handlers.rs. Используйте web::Data<Arc<AuthService>> и web::Data<Arc<BlogService>> для доступа к сервисам.
Аутентификация (публичные endpoints):

    POST /api/auth/register — регистрация нового пользователя:
        Принимает JSON с username, email, password.
        Хеширует пароль перед сохранением (используйте Argon2).
        Сохраняет пользователя в БД через UserRepository.
        Генерирует JWT-токен через JwtService.
        Возвращает 201 Created с JSON: {"token": "...", "user": {...}}.
        Обрабатывает ошибку UserAlreadyExists (возвращает 409 Conflict).
    POST /api/auth/login — вход в систему:
        Принимает JSON с username, password.
        Находит пользователя по username через UserRepository.
        Проверяет пароль (сравнение хеша).
        Генерирует JWT-токен.
        Возвращает 200 OK с JSON: {"token": "...", "user": {...}}.
        При неверных credentials возвращает 401 Unauthorized.

CRUD-операции с постами:

    POST /api/posts — создание поста (требует аутентификации):
    Извлекает user_id из JWT-токена через middleware.
    Принимает JSON с title, content.
    Создаёт пост через BlogService с author_id из токена.
    Возвращает 201 Created с телом поста.
    GET /api/posts/{id} — получение поста (публичный):
    Извлекает id из пути.
    Получает пост через BlogService.
    Возвращает 200 OK с телом поста или 404 Not Found.
    PUT /api/posts/{id} — обновление поста (требует аутентификации):
        Извлекает user_id из JWT-токена и id из пути.
        Проверяет, что пользователь является автором поста.
        Обновляет пост через BlogService.
        Возвращает 200 OK с обновлённым постом или 404 Not Found / 403 Forbidden.
    DELETE /api/posts/{id} — удаление поста (требует аутентификации):
        Извлекает user_id из JWT-токена и id из пути.
        Проверяет, что пользователь является автором поста.
        Удаляет пост через BlogService.
        Возвращает 204 No Content или 404 Not Found / 403 Forbidden.
    GET /api/posts — список постов (публичный, с пагинацией):
        Извлекает query-параметры limit и offset (по умолчанию limit=10, offset=0).
        Получает список постов через BlogService.
        Возвращает 200 OK с JSON: {"posts": [...], "total": N, "limit": 10, "offset": 0}.

JWT Middleware для actix-web. Реализуйте middleware для проверки JWT-токена в src/presentation/middleware.rs:

    Определите структуру AuthenticatedUser с полями user_id и username.
    Создайте функцию jwt_validator:
        Принимает ServiceRequest и BearerAuth (извлечённый токен).
        Получает JwtService из app_data или через замыкание (если app_data недоступен в вложенных scopes).
        Проверяет токен через JwtService::verify_token.
        Извлекает claims и вставляет AuthenticatedUser в req.extensions_mut().
        Возвращает ServiceRequest или ошибку 401 Unauthorized.
    В main.rs настройте защиту маршрутов:
        Создайте middleware через HttpAuthentication::bearer(jwt_validator).
        Примените middleware только к защищённым маршрутам (create, update, delete).
        Публичные маршруты (list, get) оставьте без middleware.
        Используйте web::scope для группировки маршрутов.

gRPC-сервис. Реализуйте все методы из proto/blog.proto в presentation/grpc_service.rs:

    Создайте структуру BlogGrpcService, которая реализует trait BlogService из сгенерированного кода.
    Используйте тот же BlogService и AuthService из application слоя, что и для HTTP handlers, чтобы избежать дублирования логики.
    Для каждого метода:
        Извлекайте JWT-токен из метаданных запроса: request.metadata().get("authorization").
        Токен передаётся в формате Bearer <token>, удалите префикс "Bearer".
        Для защищённых методов (CreatePost, UpdatePost, DeletePost) проверяйте токен через JwtService.
        Извлекайте user_id из claims-токена.
        Вызывайте соответствующие методы AuthService или BlogService.
        Преобразуйте доменные модели в protobuf типы.
        Возвращайте правильные gRPC-статусы:
            Status::ok() — успешная операция;
            Status::unauthenticated() — невалидный или отсутствующий токен;
            Status::not_found() — ресурс не найден;
            Status::invalid_argument() — невалидные данные;
            Status::already_exists() — пользователь уже существует.

Настройка CORS. В main.rs настройте CORS для работы с WASM-фронтендом:

    Используйте actix_cors::Cors.
    Для разработки можно использовать .allow_any_origin(), но в продакшене используйте whitelist origins.
    Разрешите методы: GET, POST, PUT, DELETE, OPTIONS.
    Разрешите все заголовки через .allow_any_header().
    Установите max_age для кеширования preflight-запросов.

Запуск сервера. В main.rs настройте запуск обоих серверов:

    Инициализируйте логирование через tracing-subscriber.
    Загрузите переменные окружения через dotenvy::dotenv().
    Подключитесь к БД:
        Получите DATABASE_URL из переменных окружения.
        Создайте connection pool.
        Примените миграции.
    Инициализируйте сервисы:
        Создайте JwtService из JWT_SECRET.
        Создайте репозитории (PostgresUserRepository, PostgresPostRepository).
        Создайте AuthService и BlogService.
        Оберните сервисы в Arc для разделения между HTTP и gRPC handlers.
    Настройте HTTP-сервер (actix-web):
        Создайте HttpServer с конфигурацией приложения.
        Добавьте CORS middleware.
        Настройте маршруты: публичные (/api/auth/*, GET /api/posts) и защищённые (POST/PUT/DELETE /api/posts).
        Привяжите к адресу 0.0.0.0:3000 (или другому порту).
    Настройте gRPC-сервер (tonic):
        Создайте Server::builder().
        Добавьте BlogServiceServer с вашей реализацией.
        Привяжите к адресу 0.0.0.0:50051.
    Запустите оба сервера параллельно:
        Используйте tokio::select! для параллельного выполнения.
        Обработайте ошибки завершения любого из серверов.

Переменные окружения. Создайте файл .env в корне blog-server:

    DATABASE_URL — строка подключения к PostgreSQL (например, postgres://user:password@localhost/blog_db).
    JWT_SECRET — секретный ключ для подписи JWT-токенов (минимум 32 символа).
    Не коммитьте .env в git! Добавьте его в .gitignore.

# Шаг 3. Реализация веб-сервера (blog-server)
Время реализовать приложение, которое будет выполнять роль сервера блога. Тут вы будете держать HTTP API и gRPC API. По сути, это и есть ваш бэкенд. Сюда будут входить миграции, работа с БД через sqlx, Middleware, и безопасность.
Требования:

    HTTP API на порте 3000 (actix-web) — можно изменить на 8080, если нужно.
    gRPC API на порте 50051 (tonic).
    Хранение данных в PostgreSQL через sqlx.
    Миграции для создания таблиц users и posts (используйте BIGSERIAL/BIGINT для ID, чтобы соответствовать Rust i64).
    JWT-аутентификация для защиты операций создания, редактирования и удаления постов.
    Обработка ошибок через кастомные типы ошибок (thiserror).
    Логирование через tracing.
    CORS настроен для работы с WASM-фронтендом.

Настройка зависимостей. Добавьте в blog-server/Cargo.toml необходимые зависимости:

    HTTP-сервер: actix-web, actix-web-httpauth, actix-cors.
    gRPC: tonic, tonic-build (в build-dependencies), prost, prost-types.
    База данных: sqlx с features для PostgreSQL, dotenvy для переменных окружения.
    JWT и безопасность: jsonwebtoken, argon2 для хеширования паролей.
    Утилиты: serde, serde_json, chrono, anyhow, thiserror, tracing, tracing-subscriber.
    Используйте workspace-зависимости, где возможно.

Структура проекта. Используйте clean architecture из предыдущих уроков:
```
blog-server/
├── Cargo.toml
├── build.rs
├── migrations/
│   ├── 20250101000001_create_users.sql
│   └── 20250101000002_create_posts.sql
├── proto/
│   └── blog.proto
└── src/
├── main.rs
├── domain/
│   ├── mod.rs
│   ├── post.rs          # Доменная модель Post
│   ├── user.rs          # Доменная модель User
│   └── error.rs         # Доменные ошибки
├── application/
│   ├── mod.rs
│   ├── auth_service.rs  # Сервис аутентификации
│   └── blog_service.rs  # Бизнес-логика блога
├── data/
│   ├── mod.rs
│   ├── user_repository.rs  # Репозиторий пользователей
│   └── post_repository.rs  # Репозиторий постов
├── infrastructure/
│   ├── mod.rs
│   ├── database.rs      # Подключение к БД
│   ├── jwt.rs          # Работа с JWT токенами
│   └── logging.rs       # Настройка tracing
└── presentation/
├── mod.rs
├── middleware.rs    # JWT middleware для actix-web
├── http_handlers.rs  # HTTP handlers для actix-web
└── grpc_service.rs  # gRPC сервис для tonic
```

Доменные модели. Создайте доменные модели в src/domain/:

    В user.rs определите структуры:
        User — пользователь с полями: id, username, email, password_hash, created_at.
        Структуры для запросов: регистрация (username, email, password) и вход (username, password).
        Используйте serde для сериализации/десериализации.
    В post.rs определите структуры:
        Post — пост с полями: id, title, content, author_id, created_at, updated_at.
        Структуры для запросов: создание (title, content) и обновление (title, content).
        Реализуйте метод new для создания нового поста.
    В error.rs определите кастомные типы ошибок через thiserror:
        UserNotFound, UserAlreadyExists, InvalidCredentials.
        PostNotFound, Forbidden (для случаев, когда пользователь пытается изменить чужой пост).
        Другие необходимые ошибки.

Миграции:

    Создайте файл blog-server/migrations/20250101000001_create_users.sql:
        Создайте таблицу users с полями: id (BIGSERIAL PRIMARY KEY), username (VARCHAR, UNIQUE), email (VARCHAR, UNIQUE), password_hash (VARCHAR), created_at (TIMESTAMP WITH TIME ZONE).
        Добавьте индексы на username и email для быстрого поиска.
    Создайте файл blog-server/migrations/20250101000002_create_posts.sql:
        Создайте таблицу posts с полями: id (BIGSERIAL PRIMARY KEY), title (VARCHAR), content (TEXT), author_id (BIGINT, FOREIGN KEY на users.id), created_at, updated_at (TIMESTAMP WITH TIME ZONE).
        Добавьте внешний ключ на users(id) с ON DELETE CASCADE.
        Добавьте индексы на created_at и author_id.
    Настройте автоматическое применение миграций:
        В src/infrastructure/database.rs создайте функцию для подключения к БД (connection pool).
        Создайте функцию для применения миграций через sqlx::migrate!() макрос.
        Вызовите эти функции в main.rs при старте сервера.

Настройка build.rs для gRPC. Создайте blog-server/build.rs:

    Используйте tonic_build::configure() для настройки генерации кода.
    Включите генерацию сервера и клиента (.build_server(true).build_client(true)).
    Укажите путь к proto-файлу и директорию с proto-файлами.
    Добавьте println!("cargo:rerun-if-changed=proto/blog.proto") для автоматической пересборки при изменении proto-файла.

JWT-аутентификация. Реализуйте модуль для работы с JWT-токенами в src/infrastructure/jwt.rs:

    Определите структуру Claims с полями: user_id, username, exp (время истечения).
    Создайте структуру JwtService с полями для encoding и decoding ключей.
    Реализуйте методы:
        new(secret: &str) — создание сервиса из секретного ключа.
        generate_token(user_id, username) — генерация JWT-токена с временем жизни 24 часа.
        verify_token(token) — проверка и декодирование токена.
    Используйте библиотеку jsonwebtoken для работы с токенами.

Настройка подключения к БД. Создайте src/infrastructure/database.rs:

    Реализуйте функцию create_pool(database_url):
        Создайте connection pool через PgPoolOptions.
        Настройте максимальное количество соединений (например, 5).
        Верните PgPool.
    Реализуйте функцию run_migrations(pool):
        Используйте макрос sqlx::migrate!() для указания пути к миграциям.
        Примените миграции к пулу соединений.
        Верните результат операции.

HTTP API endpoints. Реализуйте следующие endpoints в presentation/http_handlers.rs. Используйте web::Data<Arc<AuthService>> и web::Data<Arc<BlogService>> для доступа к сервисам.
Аутентификация (публичные endpoints):

    POST /api/auth/register — регистрация нового пользователя:
        Принимает JSON с username, email, password.
        Хеширует пароль перед сохранением (используйте Argon2).
        Сохраняет пользователя в БД через UserRepository.
        Генерирует JWT-токен через JwtService.
        Возвращает 201 Created с JSON: {"token": "...", "user": {...}}.
        Обрабатывает ошибку UserAlreadyExists (возвращает 409 Conflict).
    POST /api/auth/login — вход в систему:
        Принимает JSON с username, password.
        Находит пользователя по username через UserRepository.
        Проверяет пароль (сравнение хеша).
        Генерирует JWT-токен.
        Возвращает 200 OK с JSON: {"token": "...", "user": {...}}.
        При неверных credentials возвращает 401 Unauthorized.

CRUD-операции с постами:

    POST /api/posts — создание поста (требует аутентификации):
    Извлекает user_id из JWT-токена через middleware.
    Принимает JSON с title, content.
    Создаёт пост через BlogService с author_id из токена.
    Возвращает 201 Created с телом поста.
    GET /api/posts/{id} — получение поста (публичный):
    Извлекает id из пути.
    Получает пост через BlogService.
    Возвращает 200 OK с телом поста или 404 Not Found.
    PUT /api/posts/{id} — обновление поста (требует аутентификации):
        Извлекает user_id из JWT-токена и id из пути.
        Проверяет, что пользователь является автором поста.
        Обновляет пост через BlogService.
        Возвращает 200 OK с обновлённым постом или 404 Not Found / 403 Forbidden.
    DELETE /api/posts/{id} — удаление поста (требует аутентификации):
        Извлекает user_id из JWT-токена и id из пути.
        Проверяет, что пользователь является автором поста.
        Удаляет пост через BlogService.
        Возвращает 204 No Content или 404 Not Found / 403 Forbidden.
    GET /api/posts — список постов (публичный, с пагинацией):
        Извлекает query-параметры limit и offset (по умолчанию limit=10, offset=0).
        Получает список постов через BlogService.
        Возвращает 200 OK с JSON: {"posts": [...], "total": N, "limit": 10, "offset": 0}.

JWT Middleware для actix-web. Реализуйте middleware для проверки JWT-токена в src/presentation/middleware.rs:

    Определите структуру AuthenticatedUser с полями user_id и username.
    Создайте функцию jwt_validator:
        Принимает ServiceRequest и BearerAuth (извлечённый токен).
        Получает JwtService из app_data или через замыкание (если app_data недоступен в вложенных scopes).
        Проверяет токен через JwtService::verify_token.
        Извлекает claims и вставляет AuthenticatedUser в req.extensions_mut().
        Возвращает ServiceRequest или ошибку 401 Unauthorized.
    В main.rs настройте защиту маршрутов:
        Создайте middleware через HttpAuthentication::bearer(jwt_validator).
        Примените middleware только к защищённым маршрутам (create, update, delete).
        Публичные маршруты (list, get) оставьте без middleware.
        Используйте web::scope для группировки маршрутов.

gRPC-сервис. Реализуйте все методы из proto/blog.proto в presentation/grpc_service.rs:

    Создайте структуру BlogGrpcService, которая реализует trait BlogService из сгенерированного кода.
    Используйте тот же BlogService и AuthService из application слоя, что и для HTTP handlers, чтобы избежать дублирования логики.
    Для каждого метода:
        Извлекайте JWT-токен из метаданных запроса: request.metadata().get("authorization").
        Токен передаётся в формате Bearer <token>, удалите префикс "Bearer".
        Для защищённых методов (CreatePost, UpdatePost, DeletePost) проверяйте токен через JwtService.
        Извлекайте user_id из claims-токена.
        Вызывайте соответствующие методы AuthService или BlogService.
        Преобразуйте доменные модели в protobuf типы.
        Возвращайте правильные gRPC-статусы:
            Status::ok() — успешная операция;
            Status::unauthenticated() — невалидный или отсутствующий токен;
            Status::not_found() — ресурс не найден;
            Status::invalid_argument() — невалидные данные;
            Status::already_exists() — пользователь уже существует.

Настройка CORS. В main.rs настройте CORS для работы с WASM-фронтендом:

    Используйте actix_cors::Cors.
    Для разработки можно использовать .allow_any_origin(), но в продакшене используйте whitelist origins.
    Разрешите методы: GET, POST, PUT, DELETE, OPTIONS.
    Разрешите все заголовки через .allow_any_header().
    Установите max_age для кеширования preflight-запросов.

Запуск сервера. В main.rs настройте запуск обоих серверов:

    Инициализируйте логирование через tracing-subscriber.
    Загрузите переменные окружения через dotenvy::dotenv().
    Подключитесь к БД:
        Получите DATABASE_URL из переменных окружения.
        Создайте connection pool.
        Примените миграции.
    Инициализируйте сервисы:
        Создайте JwtService из JWT_SECRET.
        Создайте репозитории (PostgresUserRepository, PostgresPostRepository).
        Создайте AuthService и BlogService.
        Оберните сервисы в Arc для разделения между HTTP и gRPC handlers.
    Настройте HTTP-сервер (actix-web):
        Создайте HttpServer с конфигурацией приложения.
        Добавьте CORS middleware.
        Настройте маршруты: публичные (/api/auth/*, GET /api/posts) и защищённые (POST/PUT/DELETE /api/posts).
        Привяжите к адресу 0.0.0.0:3000 (или другому порту).
    Настройте gRPC-сервер (tonic):
        Создайте Server::builder().
        Добавьте BlogServiceServer с вашей реализацией.
        Привяжите к адресу 0.0.0.0:50051.
    Запустите оба сервера параллельно:
        Используйте tokio::select! для параллельного выполнения.
        Обработайте ошибки завершения любого из серверов.

Переменные окружения. Создайте файл .env в корне blog-server:

    DATABASE_URL — строка подключения к PostgreSQL (например, postgres://user:password@localhost/blog_db).
    JWT_SECRET — секретный ключ для подписи JWT-токенов (минимум 32 символа).
    Не коммитьте .env в git! Добавьте его в .gitignore.

# Шаг 4. Реализация клиентской библиотеки (blog-client)
Это библиотека, которая будет использоваться CLI и WASM-фронтендом. На этом шаге важно объединить общие модели, транспорт и обработку ошибок в одном месте, чтобы остальные клиенты не дублировали код. Начните с базовой структуры модулей, затем постепенно наращивайте функциональность: сначала определите типы и транспорт, затем реализуйте методы аутентификации и CRUD, и только после этого добавляйте расширенные сценарии (пагинация, фильтры, работа с токеном).
Настройка зависимостей:

    Добавьте в blog-client/Cargo.toml необходимые зависимости:
        HTTP-клиент: reqwest с feature json.
        gRPC-клиент: tonic, tonic-build (в build-dependencies), prost, prost-types.
        Утилиты: serde, serde_json, chrono, anyhow, thiserror, tokio.
        Используйте workspace зависимости, где возможно.
    Настройте build.rs (аналогично серверу):
        Используйте tonic_build::configure() для генерации клиентского кода.
        Включите только генерацию клиента (.build_client(true)).
        Укажите путь к proto-файлу.
        Добавьте println!("cargo:rerun-if-changed=proto/blog.proto").

Структура библиотеки. Создайте модули в src/lib.rs:

    http_client — для HTTP-запросов через reqwest.
    grpc_client — для gRPC-запросов через tonic.
    error — для обработки ошибок.

Определите:

    Enum Transport с вариантами Http(String) и Grpc(String) для выбора транспорта.
    Структуру BlogClient с полями: transport, http_client (Option), grpc_client (Option), token (Option).
    Структуры AuthResponse и User для ответов аутентификации.
    Структуру Post для постов (или используйте protobuf-генерированные типы).

Реализация методов BlogClient:

    new(transport) — создание клиента с инициализацией HTTP или gRPC-соединения.
    set_token(token) и get_token() — управление JWT-токеном.
    register(username, email, password) — регистрация через HTTP или gRPC, сохранение токена.
    login(username, password) — вход через HTTP или gRPC, сохранение токена.
    create_post(title, content) — создание поста (требует токен).
    get_post(id) — получение поста.
    update_post(id, title, content) — обновление поста (требует токен).
    delete_post(id) — удаление поста (требует токен).
    list_posts(limit, offset) — список постов с пагинацией.

Обработка ошибок:
Создайте enum BlogClientError в src/error.rs:

    Варианты для HTTP-ошибок (reqwest::Error), gRPC-ошибок (tonic::Status), транспортных ошибок (tonic::transport::Error).
    Варианты для бизнес-логики: NotFound, Unauthorized, InvalidRequest(String).
    Используйте thiserror для автоматической генерации Error trait.

# Шаг 5. Реализация CLI-клиента (blog-cli)
Тут понадобится библиотека blog-client для взаимодействия с сервером. CLI служит универсальным инструментом проверки бэкенда: через него удобно прогонять сценарии регистрации, логина и CRUD без поднятого фронтенда. Сначала опишите пользовательские команды и глобальные параметры, затем подключите библиотеку, настройте сохранение токена и только после этого добавляйте дополнительные флаги, обработку ошибок и человекочитаемый вывод.
Настройка зависимостей:

    Добавьте в blog-cli/Cargo.toml:
        blog-client как зависимость из локального пути.
        clap с feature derive для парсинга аргументов командной строки.
        dotenvy для переменных окружения (опционально).
        chrono для работы с датами (опционально).
        tokio из workspace.

Структура CLI:

    Используйте clap для парсинга аргументов:
        Создайте структуру Cli с полем command (subcommand) и глобальными опциями: --grpc (флаг), --server (опциональный адрес).
        Создайте enum Commands с вариантами: Register, Login, Create, Get, Update, Delete, List.
        Для каждого варианта определите необходимые аргументы (username, email, password, id, title, content, limit, offset).
    В функции main:
        Распарсите аргументы через Cli::parse().
        Определите адрес сервера (по умолчанию localhost:8080 для HTTP или localhost:50051 для gRPC).
        Создайте BlogClient с выбранным транспортом.
        Попытайтесь загрузить сохранённый токен из файла .blog_token.
        Выполните соответствующую команду через BlogClient.
        Для Register и Login сохраните полученный токен в файл .blog_token.
        Выведите результат операции.

Примеры использования. CLI должен поддерживать следующие команды:

    blog-cli register --username "ivan" --email "ivan@example.com" --password "secret123".
    blog-cli login --username "ivan" --password "secret123".
    blog-cli create --title "Мой первый пост" --content "Содержание".
    blog-cli create --title "Мой первый пост" --content "Содержание" --grpc (для gRPC).
    blog-cli get --id 1.
    blog-cli update --id 1 --title "Обновлённый заголовок".
    blog-cli delete --id 1.
    blog-cli list --limit 20 --offset 0.

# Шаг 6. Реализация WASM-фронтенда (blog-wasm)
Это фронтенд-приложение, которое компилируется в WebAssembly и работает в браузере. На этом шаге нужно связать визуальную часть с HTTP API: определите состояние приложения (пользователь, токен, список постов), продумайте сценарии входа/выхода и убедитесь, что все запросы повторяют логику серверных эндпоинтов.
После базовой функциональности можно добавить анимации, темы и другие улучшения, но критично сначала реализовать полный пользовательский поток. Пост можно опубликовать только после авторизации, с помощью полученного JWT-токена.
Вместо wasm-bindgen вы можете использовать один из Rust-фреймворков для WASM-фронтенда: Yew, Leptos, Dioxus или egui. Эти фреймворки упрощают работу с состоянием, компонентами и рендерингом, но требуют изучения их API. Если вы хотите разобраться с современными подходами к Rust-фронтенду, это отличная возможность. Для базовой реализации достаточно wasm-bindgen, но использование фреймворка может значительно улучшить структуру кода и опыт разработки.
Настройка проекта. Добавьте в blog-wasm/Cargo.toml зависимости:

    wasm-bindgen и wasm-bindgen-futures для работы с JavaScript.
    web-sys с features: Document, Element, HtmlElement, Window, Request, RequestInit, Response, Headers, console.
    gloo-net для HTTP-запросов (версия 0.6).
    serde, serde-wasm-bindgen, serde_json для сериализации.
    js-sys для работы с JavaScript-типами.

В браузере gRPC обычно не используется из-за ограничений, поэтому WASM-фронтенд должен использовать только HTTP-транспорт. Вместо использования blog-client-библиотеки, делайте HTTP-запросы напрямую через gloo-net или web-sys::Request, чтобы избежать проблем с компиляцией WASM.
Основная структура:

    Создайте структуру BlogApp с аннотацией #[wasm_bindgen]:
        Поля для хранения URL-сервера и JWT-токена.
        Методы для работы с localStorage (сохранение/загрузка токена).
    Реализуйте методы с аннотацией #[wasm_bindgen]:
        new() — конструктор для инициализации приложения;
        register(username, email, password) — регистрация через HTTP-запрос, сохранение токена;
        login(username, password) — вход через HTTP-запрос, сохранение токена;
        load_posts() — загрузка списка постов (публичный endpoint);
        create_post(title, content) — создание поста (требует токен в заголовке Authorization);
        update_post(id, title, content) — обновление поста (требует токен);
        delete_post(id) — удаление поста (требует токен);
        is_authenticated() — проверка наличия токена.
    Все методы должны возвращать Result<JsValue, JsValue> для работы с JavaScript:
        Используйте serde_wasm_bindgen::to_value() для сериализации ответов.
        Обрабатывайте ошибки и преобразуйте их в JsValue.

Работа с localStorage. Реализуйте функции для работы с localStorage:

    save_token_to_storage(token) — сохранение JWT-токена в localStorage под ключом "blog_token".
    get_token_from_storage() — загрузка токена из localStorage.
    Используйте web_sys::window().local_storage() для доступа к localStorage.

Минимальная функциональность. WASM-фронтенд должен поддерживать:

    Форму регистрации нового пользователя (username, email, password).
    Форму входа в систему (username, password).
    Сохранение JWT-токена в localStorage браузера.
    Список всех постов (отображается при загрузке страницы, может быть публичным).
    Форму создания нового поста (заголовок, содержание) — только для аутентифицированных пользователей.
    Редактирование существующего поста (по клику на пост) — только для автора поста.
    Удаление поста (кнопка удаления рядом с каждым постом) — только для автора поста.
    Отображение статуса аутентификации (залогинен/не залогинен).
    Базовая валидация форм (проверка на пустые поля).

HTML для интеграции. Создайте index.html в корне проекта:

    Подключите сгенерированный WASM-модуль через <script type="module">.
    Создайте формы для регистрации и входа.
    Создайте форму для создания поста.
    Создайте контейнер для отображения списка постов.
    Добавьте обработчики событий для всех форм.
    Используйте методы из BlogApp для взаимодействия с сервером.
    Обновляйте UI в зависимости от статуса аутентификации.

Сборка и запуск:

    Соберите WASM-модуль через wasm-pack build --target web.
    Запустите локальный HTTP-сервер (например, через python3 -m http.server 8000).
    Откройте http://localhost:8000 в браузере.

# Шаг 7. Документация и README
Создайте подробный README.md с:

    Описанием проекта и архитектуры: какие крейты есть, что они делают, как связаны между собой.
    Инструкциями по установке зависимостей и настройке окружения: PostgreSQL, переменные окружения, генерация JWT-ключа.
    Пошаговыми командами для сборки и запуска каждого компонента: server, client, CLI, WASM.
    Примерами реальных сценариев: curl, CLI-командами, шагами в браузере, чтобы проверяющий мог воспроизвести функциональность.

Артефакты проекта
Артефакты — это все документы, файлы, ссылки, материалы, которые вы обязаны приложить к проекту. Здесь это:

    Публичный GitHub-репозиторий с исходным кодом проекта.
    README.md с описанием проекта и инструкциями по запуску.
    Все четыре крейта: blog-server, blog-client, blog-cli, blog-wasm.
    Cargo workspace, который компилируется без ошибок.

# Чек-листы проекта
## Общие обязательные требования

    В репозитории присутствуют все четыре крейта: blog-server, blog-client, blog-cli, blog-wasm.
    Проект компилируется без ошибок (cargo build --workspace проходит успешно).
    Структура проекта логична: используется Cargo workspace, код разбит на модули.
    В корневом Cargo.toml корректно настроен workspace с четырьмя членами.
    README.md содержит описание проекта, инструкции по установке зависимостей и запуску всех компонентов.
    .gitignore настроен для игнорирования target/, .env, .sqlx/.

## Веб-сервер (blog-server)
Сборка и запуск:

    Сервер компилируется без ошибок (cargo build --bin blog-server).
    Сервер запускается командой cargo run --bin blog-server.
    HTTP-сервер слушает порт 8080.
    gRPC-сервер слушает порт 50051.
    Оба сервера запускаются одновременно без конфликтов.

Структура и архитектура:

    Использована clean architecture: domain/, application/, data/, infrastructure/, presentation/.
    Доменная модель Post определена в domain/post.rs.
    Бизнес-логика вынесена в application/blog_service.rs.
    Репозиторий реализован в data/post_repository.rs через sqlx.
    HTTP handlers в presentation/http_handlers.rs.
    gRPC-сервис в presentation/grpc_service.rs.
    Нет дублирования кода между HTTP и gRPC handlers (используется общий BlogService).

## Работа с БД:

    Миграции созданы в папке migrations/.
    Миграции применяются автоматически при старте сервера.
    Таблица users создаётся корректно с полями: id, username, email, password_hash, created_at.
    Таблица posts создаётся корректно с полями: id, title, content, author_id, created_at, updated_at.
    В таблице posts есть внешний ключ на users(id).
    Все SQL-запросы параметризованы (нет SQL-инъекций).
    Используется connection pool для работы с БД.
    Ошибки БД обрабатываются корректно (не падает при отсутствии записи).
    Пароли хранятся в виде хешей (Argon2 или bcrypt), не в открытом виде.

## Аутентификация и JWT:

    Модуль infrastructure/jwt.rs реализован для генерации и проверки JWT-токенов.
    JWT middleware реализован в presentation/middleware.rs.
    JWT middleware проверяет токен из заголовка Authorization: Bearer <token>.
    При невалидном токене возвращается 401 Unauthorized.
    JWT-токены содержат user_id и username в claims.
    Время жизни токена настроено (например, 24 часа).

## HTTP API
Аутентификация (публичные endpoints):

    POST /api/auth/register — регистрация (возвращает 201 Created с токеном).
    POST /api/auth/login — вход (возвращает 200 OK с токеном или 401 Unauthorized).
    При регистрации пароль хешируется перед сохранением в БД.
    При входе пароль проверяется через сравнение хешей.

CRUD-операции с постами:

    POST /api/posts — создание поста (требует аутентификации, возвращает 201 Created).
    GET /api/posts/{id} — получение поста (может быть публичным, возвращает 200 OK или 404 Not Found).
    PUT /api/posts/{id} — обновление поста (требует аутентификации, возвращает 200 OK или 404 Not Found).
    DELETE /api/posts/{id} — удаление поста (требует аутентификации, возвращает 204 No Content или 404 Not Found).
    GET /api/posts — список постов с пагинацией (может быть публичным, query параметры limit и offset).
    При создании поста author_id берётся из JWT-токена.
    При обновлении/удалении проверяется, что пользователь является автором поста.
    JSON-сериализация/десериализация работает корректно.
    Правильные HTTP статус-коды для всех операций.

gRPC API:

    Protobuf схема proto/blog.proto корректна и включает методы Register и Login.
    build.rs настроен для генерации кода из proto.
    Все методы из proto реализованы: Register, Login, CreatePost, GetPost, UpdatePost, DeletePost, ListPosts.
    Методы Register и Login возвращают AuthResponse с токеном и данными пользователя.
    Методы CreatePost, UpdatePost, DeletePost требуют аутентификации (проверка токена из метаданных).
    Правильные gRPC статус-коды (OK, NOT_FOUND, INVALID_ARGUMENT, UNAUTHENTICATED и т. д.).
    gRPC-сервис использует тот же AuthService и BlogService, что и HTTP handlers.

Обработка ошибок и логирование:

    Используются кастомные типы ошибок (thiserror или custom Error).
    Ошибки преобразуются в правильные HTTP-/gRPC-статусы.
    Ошибки логируются через tracing.
    Нет паник в коде (кроме оправданных случаев с понятным сообщением).

## Клиентская библиотека (blog-client)
Структура библиотеки:

    Библиотека компилируется без ошибок (cargo build --lib -p blog-client).
    Публичный API определён в src/lib.rs.
    Структура BlogClient с методами: new, register, login, set_token, get_token, create_post, get_post, update_post, delete_post, list_posts.
    Поддержка двух транспортов: Transport::Http и Transport::Grpc.
    Единый интерфейс независимо от транспорта.
    JWT-токен хранится в структуре BlogClient и передаётся в заголовках запросов.

HTTP-клиент:

    HTTP-клиент использует reqwest для отправки запросов.
    Методы register и login работают через HTTP и сохраняют токен в клиенте.
    JWT-токен передаётся в заголовке Authorization: Bearer <token> для защищённых запросов.
    Все CRUD-операции работают через HTTP с токеном в заголовках.
    Ошибки HTTP обрабатываются и преобразуются в BlogClientError.
    Тайм-ауты настроены корректно.

gRPC-клиент:

    gRPC-клиент использует tonic для отправки запросов.
    Методы register и login работают через gRPC и сохраняют токен в клиенте.
    JWT-токен передаётся в метаданных gRPC-запросов для защищённых методов.
    Все CRUD-операции работают через gRPC с токеном в метаданных.
    Ошибки gRPC обрабатываются и преобразуются в BlogClientError.
    Подключение к gRPC-серверу работает корректно.

Обработка ошибок:

    Кастомный тип ошибки BlogClientError определён.
    Ошибки преобразуются из HTTP-/gRPC-ошибок корректно.
    Понятные сообщения об ошибках для пользователя библиотеки.

## CLI-клиент (blog-cli)
Сборка и запуск:

    CLI компилируется без ошибок (cargo build --bin blog-cli).
    CLI запускается командой cargo run --bin blog-cli.
    Аргументы командной строки обрабатываются через clap.

Использование библиотеки

    CLI использует библиотеку blog-client (не дублирует код).
    По умолчанию используется HTTP-транспорт.
    Переключение на gRPC через флаг --grpc работает корректно.
    Адрес сервера можно указать через --server.

Команды:

    blog-cli register --username "..." --email "..." --password "..." — регистрация пользователя.
    blog-cli login --username "..." --password "..." — вход в систему.
    После регистрации/входа токен сохраняется в файл .blog_token.
    При запуске CLI-токен загружается из файла .blog_token (если существует).
    blog-cli create --title "..." --content "..." — создание поста (требует аутентификации).
    blog-cli get --id 1 — получение поста (публичный endpoint).
    blog-cli update --id 1 --title "..." [--content "..."] — обновление поста (требует аутентификации).
    blog-cli delete --id 1 — удаление поста (требует аутентификации).
    blog-cli list [--limit 10] [--offset 0] — список постов (публичный endpoint).
    Все команды работают с HTTP по умолчанию.
    Все команды работают с gRPC при указании флага --grpc.
    При отсутствии токена защищённые команды возвращают понятную ошибку.

Вывод и ошибки:

    Результаты операций выводятся в понятном формате.
    Ошибки выводятся в понятном формате для пользователя.
    Программа не падает при ошибках сети или сервера.

## WASM-фронтенд (blog-wasm)
Сборка:

    WASM-проект компилируется через wasm-pack build --target web без ошибок.
    В Cargo.toml указан crate-type = ["cdylib"].
    Зависимости wasm-bindgen, web-sys, serde-wasm-bindgen настроены корректно.

Функциональность:

    WASM-модуль использует библиотеку blog-client или делает HTTP-запросы напрямую.
    Форма регистрации работает (username, email, password).
    Форма входа работает (username, password).
    После регистрации/входа JWT-токен сохраняется в localStorage браузера.
    При загрузке страницы токен загружается из localStorage (если существует).
    Список всех постов отображается при загрузке страницы (может быть публичным).
    Форма создания нового поста работает (заголовок, содержание) — только для аутентифицированных пользователей.
    Редактирование существующего поста работает (по клику на пост) — только для автора поста.
    Удаление поста работает (кнопка удаления рядом с каждым постом) — только для автора поста.
    Отображается статус аутентификации (залогинен/не залогинен).
    Кнопка выхода очищает токен из localStorage и обновляет интерфейс.
    Базовая валидация форм работает (проверка на пустые поля).

Интеграция с браузером:

    HTML-файл index.html создан и настроен для загрузки WASM-модуля.
    JavaScript-код взаимодействует с WASM-модулем корректно.
    DOM обновляется при создании/редактировании/удалении постов.
    Ошибки отображаются пользователю в браузере.

Запуск:

    WASM-фронтенд запускается через локальный HTTP-сервер (например, python3 -m http.server 8000).
    Фронтенд работает в браузере без ошибок в консоли.
    Все CRUD-операции работают через браузерный интерфейс.

## Качество и архитектура кода

    Имена переменных и функций осмысленные, отражают назначение.
    Нет дублирования кода между компонентами (общие типы вынесены в библиотеку).
    В коде есть комментарии к сложным блокам (многопоточность, обработка ошибок, архитектурные решения).
    Тайм-ауты, порты и другие константы вынесены в константы, нет «магических чисел».
    Используется логирование (tracing), а не println! для служебных сообщений.
    Код компилируется без предупреждений (warnings).
    Используются идиомы Rust (Result, Option, match).
    Нет неиспользуемых импортов и переменных.

## Критерии качества работы

    Чистота и читаемость кода: осмысленные имена переменных и функций, отсутствие «магических чисел», комментарии к сложным логическим блокам.
    Обработка ошибок: код должен корректно обрабатывать сетевые ошибки, ошибки БД, невалидные данные.
    Архитектура и организация кода: логическое разделение на модули/функции, отсутствие дублирования кода.
    Использование clean architecture: разделение на domain, application, data, infrastructure, presentation.
    Следование ТЗ: реализованы все заявленные функции, включая HTTP и gRPC API, клиентскую библиотеку, CLI и WASM-фронтенд.
    Безопасность: пароли хранятся в виде хешей, все SQL-запросы параметризованы, JWT-токены валидируются корректно.
    Качество кода: использование идиом Rust (Result, Option, match), отсутствие предупреждений компилятора, нет неиспользуемых импортов.