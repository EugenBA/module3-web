use wasm_bindgen::prelude::*;

// Указываем, что эту функцию можно вызывать из JS
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Привет, {name}! Rust говорит тебе: добро пожаловать в WebAssembly.")
}
