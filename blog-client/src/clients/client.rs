//! Модуль для реализации API клиента
//!
//! Предоставляет функциональность для взаимодействия с бэкэндом

use crate::models::models::{Response};
use crate::{error::BlogClientError, transports::http_client::HttpClient};
use core::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(not(target_arch = "wasm32"))]
use crate::transports::grpc_client::grpc_client::GrpcClient;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};
#[cfg(target_arch = "wasm32")]
use crate::models::models::StorageUser;

/// ```rust
/// Enum `Transport` служет для выбора типа транспорта коммуникации http/gprs
///
/// # Варианты
///
/// - `Http(String)`
///   HTTP транспорт - URL as a `String`.
///
/// - `Grpc(String)` *(толко не лоя архитектуры `wasm32`)*
///   gRPC транспорт - URL as a `String`.
///
/// # Notes
/// - gRPC не может быть использовано для архитектуры `wasm32`.
///
/// # Examples
/// ```rust
/// use crate::Transport;
///
/// let http_transport = Transport::Http(String::from("https://api.example.com"));
/// println!("{:?}", http_transport);
///
/// #[cfg(not(target_arch = "wasm32"))]
/// {
///     let grpc_transport = Transport::Grpc(String::from("http://localhost:50051"));
///     println!("{:?}", grpc_transport);
/// }
/// ```
/// ```
#[derive(Debug, Clone)]
pub enum Transport {
    /// HTTP транспорт с указанием базового URL
    Http(String),
    /// gRPC транспорт с указанием адреса
    #[cfg(not(target_arch = "wasm32"))]
    Grpc(String),
}

impl Transport {
    /// Создает HTTP транспорт с указанием базового URL
    pub fn http(base_url: impl Into<String>) -> Self {
        Self::Http(base_url.into())
    }

    /// Создает gRPC транспорт с указанием адреса
    #[cfg(not(target_arch = "wasm32"))]
    pub fn grpc(addr: impl Into<String>) -> Self {
        Self::Grpc(addr.into())
    }
}

/// ```rust
/// структура для методов blog API.
///
/// `BlogClient` struct  представляет методы для обмена с бэкэндом Blog по API,
/// включает HTTP and gRPC коммункацию, управление токенами.
///
/// # Поля
///
/// - `transport`: транспорт API коммункации (e.g., HTTP, gRPC).
/// - `http_client`: опционально HTTP клиент.
/// - `grpc_client`: опционально gPRC клиент, не может применяться для архитектуры `wasm32`.
/// - `token`: токен авторизации.
///
///
/// # Пример
///
/// ```rust
/// use std::sync::{Arc, RwLock};
/// use my_crate::{BlogClient, Transport};
///
/// let blog_client = BlogClient {
///     transport: Transport::Http,
///     http_client: Some(Arc::new(HttpClient::new())),
///     #[cfg(not(target_arch = "wasm32"))]
///     grpc_client: Some(Arc::new(GrpcClient::new())),
///     token: Arc::new(RwLock::new(None)),
/// };
/// ```
/// ```
#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<HttpClient>>,
    #[cfg(not(target_arch = "wasm32"))]
    grpc_client: Option<Arc<GrpcClient>>,
    token: Arc<RwLock<Option<String>>>,
}

impl BlogClient {
    /// ```rust
    /// Инициализация струткуры `BlogClient`.
    ///
    ///
    /// # Параметры
    ///
    /// - `transport`: транспорт http/gPRC
    ///   - `Transport::Http(base_url)`: http клиент с url.
    ///   - `Transport::Grpc(addr)`: gprc клиент с url.
    ///
    /// - `timeout`: `Duration` таймаут операции
    ///
    /// # Возврат
    ///
    /// Returns `Result<Self, BlogClientError>`:
    /// - `Ok(Self)`: иницилизованный клиент
    /// - `Err(BlogClientError)`: ошибка
    ///
    /// # Ограничения платформы
    ///
    /// - Для архитектуры `wasm32` доступен только http клиент
    ///
    /// # Пример
    ///
    /// HTTP transport:
    /// ```rust
    /// let transport = Transport::Http("https://api.example.com".to_string());
    /// let timeout = Duration::from_secs(30);
    /// let client = BlogClient::new(transport, timeout).await?;
    ///
    ///
    /// gRPC transport (non-`wasm32`):
    /// ```rust
    /// let transport = Transport::Grpc("http://localhost:50051".to_string());
    /// let timeout = Duration::from_secs(30);
    /// let client = BlogClient::new(transport, timeout).await?;
    ///
    /// # Errors
    ///
    /// - invalid URL,
    /// - unreachable server,
    /// - configuration errors),
    /// Возращает ошибку type `BlogClientError`
    ///
    /// ```
    pub async fn new(transport: Transport, timeout: Duration) -> Result<Self, BlogClientError> {
        match &transport {
            Transport::Http(base_url) => {
                let http_client = HttpClient::new(base_url, timeout).await?;
                Ok(Self {
                    transport,
                    http_client: Some(Arc::new(http_client)),
                    #[cfg(not(target_arch = "wasm32"))]
                    grpc_client: None,
                    token: Arc::new(RwLock::new(None)),
                })
            }
            #[cfg(not(target_arch = "wasm32"))]
            Transport::Grpc(addr) => {
                let grpc_client = GrpcClient::new(addr).await?;
                Ok(Self {
                    transport,
                    http_client: None,
                    grpc_client: Some(Arc::new(grpc_client)),
                    token: Arc::new(RwLock::new(None)),
                })
            }
        }
    }
    /// ```rust
    /// Создает инстанс для взаимодействия через HTTP.
    ///
    /// # Аргументы
    ///
    /// * `base_url` - адрес сервера
    /// * `timeout` - The duration after which HTTP requests will timeout.
    ///
    /// # Возврат
    ///
    /// Возвращает BlogClient или BlogClientError
    ///
    /// # Пример
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use your_crate::BlogClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = BlogClient::http("https://example.com", Duration::from_secs(30)).await;
    ///     match client {
    ///         Ok(client) => println!("Client successfully created!"),
    ///         Err(error) => eprintln!("Failed to create client: {:?}", error),
    ///     }
    /// }
    ///
    /// # Ошибки
    ///
    /// Возвращает `BlogClientError`
    /// ```
    pub async fn http(
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, BlogClientError> {
        Self::new(Transport::Http(base_url.into()), timeout).await
    }
    /// ```rust
    /// Создает инстанс для взаимодействия через gRPC.
    ///
    /// Данный метод не доступен для wasm32 архитектуры (`not(target_arch = "wasm32")`).
    ///
    /// # Параметр
    /// - `addr`: адрес сервера
    /// - `timeout`: таймаут
    ///
    /// # Возврат
    /// - `Ok(Self)`: BlogClient
    /// - `Err(BlogClientError)`: BlogClientError
    ///
    /// # Пример
    /// ```rust
    /// use std::time::Duration;
    ///
    /// let addr = "http://localhost:50051";
    /// let timeout = Duration::from_secs(30);
    ///
    /// let client = BlogClient::grpc(addr, timeout).await?;
    ///
    ///
    /// # Ошибки
    /// возарщает ошибку BlogClient в случае невозможности инициализации
    ///
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn grpc(addr: impl Into<String>, timeout: Duration) -> Result<Self, BlogClientError> {
        Self::new(Transport::Grpc(addr.into()), timeout).await
    }

    /// ```rust
    /// Устанвливает токен
    ///
    ///
    /// # Параметр
    ///
    /// * `token` - `Option<String>` токен
    ///
    ///
    ///
    /// # Пример
    ///
    /// ```rust
    /// let client = MyClient::new();
    /// client.set_token(Some("my-auth-token".to_string())).await;
    ///
    /// ```
    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token.clone();
        if let Some(token) = token.clone() {
                let _ = self.save_token(&token);
        }
    }
    /// ```rust
    ///  Получение токена
    ///
    ///
    ///   # Возврат
    ///  - `Option<String>`: `Some(token)`  если оокен есть или `None` если токен не сохранен
    ///
    ///
    ///  # Пример
    ///
    /// let my_struct = MyStruct::new();
    ///  let token = my_struct.get_token().await;
    ///  if let Some(t) = token {
    ///     println!("Token: {}", t);
    ///  } else {
    ///     println!("No token available.");
    ///  }
    /// ```
    pub async fn get_token(&self) -> Option<String> {
        if cfg!(target_arch = "wasm32") {
            self.token.read().await.clone()
        } else {
            self.token.read().await.clone()
        }
    }
    /// ```rust
    /// Регистрация пользователя
    ///
    ///
    /// # Аргументы
    ///
    /// * `username` - имя пользователя
    /// * `email` - емайл
    /// * `password` - пароль
    ///
    /// # Возврат
    ///
    /// - `Ok(Response)` при успешной регистрации
    /// - `Err(BlogClientError)` в случае ошибки
    ///
    /// # Platform-specific Behavior
    ///
    /// * **WASM32 Target**  не используетс ятранспорт gRPC
    ///
    /// # Ошибки
    ///
    /// - BlogClientError::NoTransportConfigured`
    /// - Ошибки регистрации и коммуникаци
    ///
    /// # Пример
    ///
    /// ```rust
    /// use your_crate::BlogClient;
    ///
    /// let client = BlogClient::new_with_http();
    /// let result = client.register("john_doe", "john@example.com", "secure_password").await;
    ///
    /// match result {
    ///     Ok(response) => {
    ///         println!("User registered successfully: {:?}", response);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to register user: {:?}", e);
    ///     }
    /// }
    /// ```
    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => {
                let response = client.register(username, email, password).await?;
                self.set_token(response.token.clone()).await;
                #[cfg(target_arch = "wasm32")]
                if let Some(id) = response.id {
                    self.save_username(StorageUser {
                        id,
                        username: username.to_string(),
                    })?;
                }
                Ok(response)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.register(username, email, password).await?;
                self.set_token(response.token.clone()).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Вход пользователя
    ///
    ///
    /// # Парметры
    ///
    /// - `username`: имя пользователя
    /// - `password`: пароль
    ///
    /// # Возврат
    ///
    /// - `Ok(Response)`: успешный вход
    /// - `Err(BlogClientError)`: ошибка
    ///
    /// # Пример
    /// ```rust
    /// use your_crate::BlogClient;
    ///
    /// let client = BlogClient::new_with_http();
    /// let result = client.login("john_doe", "secure_password").await;
    ///
    /// match result {
    ///     Ok(response) => {
    ///         println!("User login successfully: {:?}", response);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to login user: {:?}", e);
    ///     }
    /// }
    /// ```
    pub async fn login(&self, username: &str, password: &str) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(response.token.clone()).await;
                #[cfg(target_arch = "wasm32")]
                if let Some(id) = response.id {
                    self.save_username(StorageUser {
                        id,
                        username: username.to_string(),
                    })?;
                }
                Ok(response)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(response.token.clone()).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    ///Созаднаие нового поста
    ///
    ///
    /// # Параметры
    ///
    /// - `title`: заголовок поста
    /// - `content`: текст поста
    ///
    /// # Возврат
    ///
    /// - `Ok(Response)`: если посто успешно создан
    /// - `Err(BlogClientError)`: ошибки при создании поста
    ///
    /// # Ошибки
    ///
    /// - `BlogClientError::NoTransportConfigured`: ошибка созадания тарнспорта
    /// - Другие при возврате ошибки с бэкэнда
    ///
    /// # Пример
    ///
    /// ```rust
    /// use my_crate::{BlogClient, BlogClientError};
    ///
    /// async fn create_new_post(client: &BlogClient) -> Result<(), BlogClientError> {
    ///     let title = "My First Blog Post";
    ///     let content = "This is the content of the blog post.";
    ///     match client.create_post(title, content).await {
    ///         Ok(response) => {
    ///             println!("Post created successfully: {:?}", response);
    ///             Ok(())
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Failed to create post: {:?}", err);
    ///             Err(err)
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn create_post(
        &self,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.create_post(title, content, self.get_token().await).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.create_post(title, content, self.get_token().await).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Получение поста
    ///
    ///
    /// # Парметры
    /// - `id` (i64): идентификатор поста
    ///
    /// # Возврат
    ///   - `Ok(Response)`: ответ с данными поста
    ///   - `Err(BlogClientError)`: ошибки
    ///
    /// # Errors
    /// Возвращает `BlogClientError
    ///
    ///
    /// # Example
    /// ```rust
    /// let response = blog_client.get_post(12345).await;
    /// match response {
    ///     Ok(post) => println!("Retrieved post: {:?}", post),
    ///     Err(e) => eprintln!("Failed to retrieve post: {:?}", e),
    /// }
    /// ```
    /// ```
    pub async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_post(id).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Обновление данных поста
    ///
    ///
    /// # Параметры
    /// - `id`: идентификатор поста
    /// - `title`: заголовок поста
    /// - `content`: текст поста
    ///
    /// # Возврат
    /// - `Ok(Response)`: в случае успешного обновления поста
    /// - `Err(BlogClientError)`: ошибки обновления поста
    ///
    /// # Ошибки
    /// Возвращает `BlogClientError
    ///
    /// # Example
    /// ```rust
    /// let client = BlogClient::new();
    /// let result = client.update_post(42, "Updated Title", "Updated content").await;
    /// match result {
    ///     Ok(response) => println!("Post updated successfully: {:?}", response),
    ///     Err(e) => eprintln!("Failed to update post: {:?}", e),
    /// }
    /// ```
    /// ```
    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.update_post(id, title, content, self.get_token().await).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.update_post(id, title, content, self.get_token().await).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Удалени поста
    ///
    ///
    /// # Параметры
    ///
    /// * `id` - идетификатор поста
    ///
    /// # Возврат
    /// * `Ok(Response)` - в случае успешногоп удаления поста
    /// * `Err(BlogClientError)` - в случе вознкновения ошибки
    ///
    ///
    /// # Ошибки
    /// Возвращает `BlogClientError
    ///
    /// # Пример
    ///
    /// ```rust
    /// use my_blog_client::BlogClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BlogClient::new();
    /// let post_id: i64 = 12345;
    ///
    /// match client.delete_post(post_id).await {
    ///     Ok(response) => println!("Post deleted successfully: {:?}", response),
    ///     Err(err) => eprintln!("Failed to delete post: {:?}", err),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// ```
    pub async fn delete_post(&self, id: i64) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.delete_post(id, self.get_token().await).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.delete_post(id, self.get_token().await).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Возврат списка постов
    ///
    ///
    /// # Параметры
    /// - `limit`: ограничени числа постов
    /// - `offset`:  смещения от начала списка постов
    ///
    /// # Возврат
    /// - `Ok(Response)`: в случае успешного получения списка
    /// - `Err(BlogClientError)`: в случае ошибки
    ///
    ///
    /// # Errors
    /// - Возвращает `BlogClientError
    ///
    /// # Examples
    /// ```rust
    /// let limit = Some(10);
    /// let offset = Some(0);
    ///
    /// match client.list_posts(limit, offset).await {
    ///     Ok(response) => println!("Retrieved posts: {:?}", response),
    ///     Err(err) => println!("Error fetching posts: {:?}", err),
    /// }
    /// ```
    /// ```
    pub async fn list_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }
    /// ```rust
    /// Возвращает транспорт  HTTP или gRPC
    ///
    /// # Returns
    /// Возвращает тип Transport
    ///
    /// # Examples
    /// ```rust
    /// let obj = MyStruct { transport: Transport::new() };
    /// let transport = obj.transport();
    /// ```
    /// ```
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
    /// ```rust
    /// Загрузка токена
    ///
    ///
    /// # Возврат
    /// - `Ok(())` токен считан из хранилища
    /// - `Err(BlogClientError)` при возникновении ошибки
    ///
    /// # Ошибки
    /// Возвращает `BlogClientError
    ///
    /// # Пример
    /// ```rust
    /// // Assuming `client` is an instance that implements the `load_token` method:
    /// client.load_token().await?;
    /// ```
    /// ```
    pub async fn load_token(&self) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let token_file = ".blog_token";
            if Path::new(token_file).exists() {
                let token = fs::read_to_string(token_file)?;
                self.set_token(Some(token)).await;
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let token = LocalStorage::get("blog_token")?;
            self.set_token(Some(token)).await;
        }
        Ok(())
    }
    /// ```rust
    /// Сохраняет токен в хранилища
    ///
    ///
    /// # Парметры
    ///
    /// * `token` - токен
    ///
    /// # Returns
    ///
    /// * `Ok(())` если сохранение успешно
    /// * `Err(BlogClientError)` в случае ошибки
    ///
    ///
    /// # Пример
    /// ```rust
    /// # use your_crate::BlogClient; // Replace with your actual module path
    /// let client = BlogClient::new();
    /// client.save_token("my_secure_token").expect("Failed to save token");
    /// ```
    /// ```
    pub fn save_token(&self, token: &str) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            fs::write(".blog_token", token)?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::set("blog_token", token)?;
            self.set_token(Some(token.to_string()));
            Ok(())
        }
    }
    /// ```rust
    ///
    ///  Сохранение имени пользователя в хранилище только для архитектруры wasm32
    ///
    ///
    /// # Возврат
    ///  - `Ok(())` в случае успешного сохранения
    ///  - `Err(BlogClientError)` в случае ошибки
    ///
    /// # Ошибки
    /// Возвращает `BlogClientError
    ///
    /// # Пример
    /// ```rust
    ///
    /// match client.save_username(user) {
    ///  Ok(_) => println!("Username saved successfully!"),
    ///  Err(err) => eprintln!("Failed to save username: {}", err),
    ///  }
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub fn save_username(&self, user: StorageUser) -> Result<(), BlogClientError> {
        LocalStorage::set("blog_username", &user)?;
        Ok(())
    }

    /// ```rust
    /// Очистка данных аутентификации
    ///
    /// # Возврат
    /// -Ok(())` в случае успешного сохранения
    /// -`Err(BlogClientError)` в случае ошибки
    ///
    /// # Ошибка
    /// Возвращает `BlogClientError
    ///
    /// # Пример
    /// ```rust
    /// let client = BlogClient::new();
    ///
    /// if let Err(e) = client.clear_token() {
    ///     eprintln!("Failed to clear token: {:?}", e);
    /// } else {
    ///     println!("Token cleared successfully.");
    /// }
    /// ```
    pub fn clear_token(&self) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let token_file = ".blog_token";
            if Path::new(token_file).exists() {
                fs::remove_file(".blog_token")?;
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::delete("blog_token");
            LocalStorage::delete("blog_username");
        }
        Ok(())
    }
    /// ```rust
    /// Получение имени пользователя
    ///
    ///
    /// # Возврат
    /// - `Some(StorageUser)` данные сохраненного пользователя
    /// - `None` если данных нет
    ///
    /// # Example
    /// ```rust
    /// #[cfg(target_arch = "wasm32")]
    /// {
    ///     if let Some(user) = your_instance.get_user() {
    ///         // Use the retrieved StorageUser object
    ///         println!("Username: {:?}", user);
    ///     } else {
    ///         println!("No user found in local storage.");
    ///     }
    /// }
    /// ```
    ///
    #[cfg(target_arch = "wasm32")]
    pub fn get_user(&self) -> Option<StorageUser> {
        
        LocalStorage::get("blog_username").ok()
    }
}