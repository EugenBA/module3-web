#![warn(missing_docs)]

use blog_client::clients::client::{BlogClient, Transport};
use blog_client::models::models::StorageUser;
use core::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct WasmBlogClient {
    pub(crate) http_client: BlogClient,
}
#[wasm_bindgen]
impl WasmBlogClient {
    /// ```
    /// Конструктор клиента
    ///
    /// # Параметры
    /// - `url` (`String`): строка подключения к серверу
    /// - `timeout_sec` (`i32`): таймаут
    ///
    /// # Возврат
    /// - `Ok(Self)` возарщает инстанс, в случае отсутсвия ошибок
    /// - `Err(JsValue)` возврат ошибок
    ///
    /// # Ошибки
    /// Возращается JsValue.
    ///
    /// ```
    #[wasm_bindgen(constructor)]
    pub async fn new(url: String, timeout_sec: i32) -> Result<Self, JsValue> {
        let transport = Transport::http(url);
        let timeout = Duration::from_secs(timeout_sec as u64);
        let http_client = BlogClient::new(transport, timeout)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        let _ = http_client.load_token().await;
        Ok(Self { http_client })
    }

    /// ```rust
    /// Регистрация пользователя
    ///
    /// # Парметры
    ///
    /// * `username` - имя пользователя
    /// * `email` - адрес электронной почты
    /// * `password` - пароль
    ///
    /// # Returns
    ///
    /// * `Ok(JsValue)` - возращает токен имя польщователя и его id
    /// * `Err(JsValue)` - возврат ошибки
    ///
    /// # Errors
    /// Возрашщает в виде JSValue
    ///
    /// ```
    #[wasm_bindgen]
    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .register(&username, &email, &password)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.token)?)
    }

    /// ```rust
    /// Вход пользователя
    ///
    /// # Парметры
    /// * `username` - имя пользователя
    /// * `password` - пароль
    ///
    /// # Возврат
    /// * Ok(JsValue) - возращает токен имя польщователя и его id
    /// * Err(JSvalue) - возврат ошибки
    ///
    /// # Ошибки
    /// * Возрвщвет JSvalue
    ///
    #[wasm_bindgen]
    pub async fn login(&self, username: String, password: String) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .login(&username, &password)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response)?)
    }

    /// ```rust
    ///
    /// Создание поста
    ///
    /// # Параметры
    ///
    /// * `title` - заголовок поста
    /// * `content` - текст поста
    ///
    /// # Возврат
    ///
    /// * `Ok(JsValue)` - возращает пост
    /// * `Err(JsValue)` - ошибкуа JSvalue
    ///
    /// # Ошибки
    ///  Djphfoftn jib,rb JSvalue
    ///
    /// ```
    #[wasm_bindgen]
    pub async fn create_post(&self, title: String, content: String) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .create_post(&title, &content)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }
    /// ```rust
    ///
    /// Обновление поста
    ///
    /// # Параметры
    ///
    /// * `id` - идентификатор поста
    /// * `title` - заголовок поста
    /// * `content` - текст поста
    ///
    /// # Возврат
    ///
    /// * `Ok(JsValue)` обновление поста
    /// *  Err(JsValue)` возврат ошибки
    ///
    /// # Ошибки
    /// Возвращает JSvalue
    ///
    /// ```
    #[wasm_bindgen]
    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .update_post(id, &title, &content)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    /// ```rust
    ///  Удалить пост
    ///
    /// # Параметры
    /// * `id` - идентификатор поста
    ///
    /// # Returns
    /// * Ok(JsValue) - пост успешно удален
    /// * Err(JsValue) - ошибка удаления поста\
    /// # Errors
    ///  Возращает JSvalue
    /// ```
    #[wasm_bindgen]
    pub async fn delete_post(&self, id: i64) -> Result<JsValue, JsValue> {
        self.http_client
            .delete_post(id)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(JsValue::from_str(
            format!("Post with id {} deleted.", id).as_str(),
        ))
    }

    /// ```rust
    /// Получить пост
    ///
    ///
    /// # Паарметры
    ///
    /// * `id` - идентификатор поста
    ///
    /// # Возврат
    ///
    /// * `Ok(JSvalue)` - в случае успеха возращает данные обновленного поста
    /// *  Err(JsValue)` -  ошибка обновления поста
    ///
    /// # Ошибки
    /// Возвращает JSvalue
    /// ```
    #[wasm_bindgen]
    pub async fn get_post(&self, id: i64) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .get_post(id)
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    /// ```rust
    /// Возвращает список постов
    ///
    /// # Параметры
    /// - `limit` (`i64`): ограничение по каоличеству постов
    /// - `offset` (`i64`): смещение от начала выборки постов
    ///
    /// # Возврат
    /// - `Ok(JsValue)`: список постов
    /// - `Err(JsValue)`: ошибки получения данных постов
    ///
    /// # Errors
    /// Возвращает JSvalue
    ///
    /// ```
    #[wasm_bindgen]
    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<JsValue, JsValue> {
        let response = self
            .http_client
            .list_posts(Some(limit), Some(offset))
            .await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    /// ```rust
    /// Выход пользователя
    ///
    /// # Возврат
    ///
    ///  * `Ok(JsValue)` - выход пользователя успешен (данные регистрации удалены)
    ///  * `Err(JsValue)` - ошибка удаления данных регистрации из хранилища
    /// # Errors
    ///  Возвращает JSvalue
    ///
    ///
    /// ```
    #[wasm_bindgen]
    pub fn logout(&self) -> Result<JsValue, JsValue> {
        let _ = self
            .http_client
            .clear_token()
            .map_err(|e| JsValue::from_str(&format!("{}", e)));
        Ok(JsValue::from_str("Logout successful."))
    }

    /// ```rust
    ///
    /// Проверка, что пользователь зарегестрировал
    ///
    ///  # Returns
    /// * `bool` - возвращает true если пользователь зарегестрировался
    ///
    /// ```
    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> bool {
        self.http_client.get_user().is_some()
    }
    /// ```rust
    /// Retrieves the current user's information.
    ///
    /// This function attempts to get the current user's data from the HTTP client.
    /// If the HTTP client fails to fetch the user, a default `StorageUser` instance
    /// is used with an `id` of `-1` and an empty `username`. The user's data is
    /// then serialized into a `JsValue` using `serde_wasm_bindgen`.
    ///
    /// # Returns
    /// - `Ok(JsValue)` containing the serialized user data if successful.
    /// - `Err(JsValue)` if serialization into `JsValue` fails.
    ///
    /// # Errors
    /// This function can return an error if the serialization of the `StorageUser`
    /// struct to `JsValue` fails.
    ///
    /// # Example
    /// ```
    /// let current_user = get_current_user();
    /// match current_user {
    ///     Ok(user) => {
    ///         // Use the user data
    ///     }
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    /// ```
    #[wasm_bindgen]
    pub fn get_current_user(&self) -> Result<JsValue, JsValue> {
        let user = self.http_client.get_user().unwrap_or_else(|| StorageUser {
            id: -1,
            username: "".to_string(),
        });
        Ok(serde_wasm_bindgen::to_value(&user)?)
    }
}
/// ```rust
/// обработка паники
/// ```
#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
