
use wasm_bindgen::prelude::*;
use blog_client::clients::client::{BlogClient, Transport};
use blog_client::models::models::StorageUser;
use core::time::Duration;
use log::{info, error, warn, debug, trace};

// Указываем, что эту функцию можно вызывать из JS
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Привет, {name}! Rust говорит тебе: добро пожаловать в WebAssembly.")
}

#[wasm_bindgen]
struct WasmBlogClient{
    pub(crate) http_client: BlogClient
}
#[wasm_bindgen]
impl WasmBlogClient {

    #[wasm_bindgen(constructor)]
    pub async fn new(url: String, timeout_sec: i32) -> Result<Self, JsValue> {
        let transport = Transport::http(url);
        let timeout = Duration::from_secs(timeout_sec as u64);
        let http_client = BlogClient::new(transport, timeout)
            .await.map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        let _ = http_client.load_token().await;
        wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
        Ok(Self { http_client })
    }
    #[wasm_bindgen]
    pub async fn register(&self, username: String, email: String, password: String) -> Result<JsValue, JsValue> {
        let response = self.http_client.register(&username, &email, &password).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.token)?)
    }
    #[wasm_bindgen]
    pub async fn login(&self, username: String, password: String) -> Result<JsValue, JsValue> {
        trace!("login called with username: {}, password: {}", username, password);
        let response = self.http_client.login(&username, &password).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        trace!("{:?}", response);
        Ok(serde_wasm_bindgen::to_value(&response)?)
    }

    #[wasm_bindgen]
    pub async fn create_post(&self, title: String, content: String) -> Result<JsValue, JsValue> {
        let response = self.http_client.create_post(&title, &content).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }
    #[wasm_bindgen]
    pub async fn update_post(&self, id: i64, title: String, content: String) -> Result<JsValue, JsValue> {
        let response = self.http_client.update_post(id, &title, &content).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    #[wasm_bindgen]
    pub async fn delete_post(&self, id: i64) -> Result<JsValue, JsValue> {
        let _ = self.http_client.delete_post(id).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(JsValue::from_str("ok"))
    }

    #[wasm_bindgen]
    pub async fn get_post(&self, id: i64) -> Result<JsValue, JsValue> {
        let response = self.http_client.get_post(id).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    #[wasm_bindgen]
    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<JsValue, JsValue> {
        let response = self.http_client.list_posts(Some(limit), Some(offset)).await
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(serde_wasm_bindgen::to_value(&response.posts)?)
    }

    #[wasm_bindgen]
    pub fn logout(&self) -> Result<JsValue, JsValue> {
        let _ = self.http_client.clear_token().map_err(|e| JsValue::from_str(&format!("{}", e)));
        Ok(JsValue::from_str("Logout successful."))
    }

    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> bool {
        self.http_client.get_user().is_some()
    }

    #[wasm_bindgen]
    pub fn get_current_user(&self) -> Result<JsValue, JsValue> {
       let user = self.http_client.get_user().unwrap_or_else(|| StorageUser{
           id: -1,
           username: "".to_string()
       });
        Ok(serde_wasm_bindgen::to_value(&user)?)
    }
}
#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}