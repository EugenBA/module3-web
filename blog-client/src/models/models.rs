//! Модуль для взаимодействия с бэкэндом
//!
//! Предоставляет структуры для взаимодействия с бэкэндом
//!
#[cfg(not(target_arch = "wasm32"))]
use crate::blog::ProtoPost;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

/// ```
/// Представляет поля для пользователя
///
/// # Трейты
///
///
/// * `Debug`
/// * `Clone`
/// * `Serialize`
/// * `Deserialize`
///
/// # Пример
///
/// use chrono::Utc;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// pub struct User {
///     pub id: i64,
///     pub username: String,
///     pub email: String,
///     pub created_at: DateTime<Utc>,
/// }
///
/// let user = User {
///     id: 1,
///     username: "johndoe".to_string(),
///     email: "johndoe@example.com".to_string(),
///     created_at: Utc::now(),
/// };
///
/// println!("{:?}", user);
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// ID пользователя
    pub id: i64,
    /// Имя пользователя
    pub username: String,
    /// Электроная почта
    pub email: String,
    /// дата создания
    pub created_at: DateTime<Utc>,
}

/// ```rust
///
/// Структура предосталяет данные поста
///
/// # Трейты
///
/// * `Debug`
/// * `Clone`
/// * `Serialize`
/// * `Deserialize`
///
/// # Пример
///
/// ```rust
/// use chrono::Utc;
/// use your_crate_name::Post; // Replace `your_crate_name` with the correct module or crate name.
///
/// let post = Post {
///     id: 1,
///     title: String::from("My First Post"),
///     content: String::from("This is the content of my first post."),
///     author_id: 123,
///     created_at: Utc::now(),
///     updated_at: None,
/// };
///
/// println!("{:?}", post);
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// идентификато поста
    pub id: i64,
    /// заголовок
    pub title: String,
    /// тест поста
    pub content: String,
    /// идентификатор автора
    pub author_id: i64,
    /// дата создания
    pub created_at: DateTime<Utc>,
    /// дата корректировки
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(not(target_arch = "wasm32"))]
impl Post {
    fn timestamp_to_chrono(timestamp: Option<prost_types::Timestamp>) -> DateTime<Utc> {
        if let Some(ts) = timestamp {
            let system_time = std::time::UNIX_EPOCH
                + Duration::from_secs(ts.seconds as u64)
                + Duration::from_nanos(ts.nanos as u64);
            DateTime::<Utc>::from(system_time)
        } else {
            DateTime::<Utc>::default()
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl From<ProtoPost> for Post {
    fn from(value: ProtoPost) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: Post::timestamp_to_chrono(value.created_at),
            updated_at: Some(Post::timestamp_to_chrono(value.updated_at)),
        }
    }
}

/// ```
/// Структура для данных ответа
///
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Response {
    /// список постов
    pub posts: Option<Vec<Post>>,
    /// идентификатор
    pub id: Option<i64>,
    /// имя пользователя
    pub user: Option<String>,
    /// токен
    pub token: Option<String>,
    /// заголовок
    pub title: Option<String>,
    /// текст поста
    pub content: Option<String>,
}
#[cfg(not(target_arch = "wasm32"))]
impl From<ProtoPost> for Response {
    fn from(value: ProtoPost) -> Self {
        Self {
            posts: None,
            id: Some(value.id),
            user: None,
            token: None,
            title: Some(value.title),
            content: Some(value.content),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Response {
    /// ```rust
    /// Форматирование вывода данных cli
    ///
    ///
    /// # Возврат
    ///
    /// Тип `String`
    ///
    ///
    /// # Пример
    ///
    /// ```rust
    /// let output = user.format_output();
    /// println!("{}", output);
    ///
    /// ```plaintext
    /// User ID: 123
    /// User name: John Doe
    /// Posts:
    /// ------------------------------
    /// Post ID: 1
    /// Title: My First Post
    /// Content: This is the content of the first post.
    /// ------------------------------
    /// Post ID: 2
    /// Title: Second Post
    /// Content: Another post's content.
    /// ------------------------------
    /// ```
    /// ```
    pub fn format_output(&self) -> String {
        let mut output = String::new();
        if let Some(posts) = &self.posts {
            if let Some(id) = &self.id {
                output.push_str(&format!("User ID: {}\n", id));
            }
            if let Some(user) = &self.user {
                output.push_str(&format!("User name: {}\n", user));
            }
            output.push_str("Posts:\n");
            output.push_str("------------------------------\n");
            for post in posts {
                output.push_str(&format!(
                    "Post ID: {}\nTitle: {}\nContent: {}\n",
                    post.id, post.title, post.content
                ));
                output.push_str("------------------------------\n");
            }
        } else {
            if let Some(id) = &self.id
                && let Some(title) = &self.title
                && let Some(content) = &self.content
            {
                output.push_str(&format!(
                    "Post ID: {}\nTitle: {}\nContent: {}\n",
                    id, title, content
                ));
            } else {
                output.push_str("No posts found\n");
            }
        }
        output
    }
}

/// ```
/// Структура для запроса регистрации пользователя
///
///
/// # Трайты
///
/// * `Debug`
/// * `Clone`
/// * `Serialize`
/// * `Deserialize`
///
/// # Пример
///
/// use your_crate::RegisterUserRequest;
///
/// let request = RegisterUserRequest {
///     username: String::from("example_user"),
///     email: String::from("user@example.com"),
///     password: String::from("securepassword123"),
/// };
///
/// println!("{:?}", request);
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    /// имя пользователя
    pub username: String,
    /// email
    pub email: String,
    /// пароль
    pub password: String,
}

/// ```rust
/// Структура для данных запроса входа
///
///
/// # Пример
/// use serde_json;
/// use your_crate::LoginRequest;
///
/// let login_request = LoginRequest {
///     username: String::from("JohnDoe"),
///     password: String::from("SuperSecret123"),
/// };
///
/// // Serialize to JSON
/// let json = serde_json::to_string(&login_request).unwrap();
/// assert!(json.contains("JohnDoe"));
///
/// // Deserialize from JSON
/// let deserialized: LoginRequest = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized.username, "JohnDoe");
/// assert_eq!(deserialized.password, "SuperSecret123");
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// имя пользователя
    pub username: String,
    /// пароль
    pub password: String,
}

/// ```rust
/// Структура для запроса создания поста
///
///
/// # Трейты
///
/// * `Debug` - Allows formatting of the structure for debugging purposes.
/// * `Clone` - Enables deep copying of the structure.
/// * `Serialize` - Allows the structure to be serialized (e.g., for converting into JSON).
/// * `Deserialize` - Allows the structure to be deserialized (e.g., for converting from JSON).
///
/// # Пример
///
///
/// ```rust
/// let create_post_request = CreatePostRequest {
///     title: String::from("My First Post"),
///     content: String::from("This is the content of my first post."),
/// };
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    /// заголовок поста
    pub title: String,
    /// тест поста
    pub content: String,
}

/// ```rust
/// Структура запроса на обновления поста
///
/// # Трейты
///
/// The `UpdatePostRequest` struct derives the following traits:
///
/// * `Debug`
/// * `Clone`
/// * `Serialize`
/// * `Deserialize`
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    /// идентификатор
    pub id: i64,
    /// заголовок поста
    pub title: String,
    /// текст поста
    pub content: String,
}

/// ```
/// Структура ответа списка постов
///
///
/// # Трейты
///
/// - `Debug`
/// - `Clone`
/// - `Serialize`
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsResponse {
    /// список постов
    pub posts: Vec<Post>,
    /// общее количество постов
    pub total: Option<u64>,
    /// смещение
    pub offset: i64,
    /// ограничение вывода
    pub limit: i64,
}

/// ```
/// Структура для запроса списка постов
///
///
/// # Пример
///
/// use your_crate::ListPostsRequest;
///
/// let request = ListPostsRequest {
///     offset: Some(10),
///     limit: Some(20),
///     author_id: Some("author123".to_string()),
/// };
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsRequest {
    /// смещения списка постов
    pub offset: Option<i64>,
    /// ограничение выборки
    pub limit: Option<i64>,
    /// идентификатор автора поста
    pub author_id: Option<String>,
}

/// ```rust
/// Структра для храненданных в глобальном хранилище, только для архитектуры wasm32
///
///
/// # Трейты
///
/// * `Debug`
/// * `Clone`
/// * `Serialize`
/// * `Deserialize`
///
/// ```
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUser {
    /// идентификатор пользователя
    pub id: i64,
    /// имя пользователя
    pub username: String,
}
