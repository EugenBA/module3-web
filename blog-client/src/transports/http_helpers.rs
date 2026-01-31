#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Method, RequestBuilder, Client};
#[cfg(target_arch = "wasm32")]
use gloo_net::http::{Request, RequestBuilder};
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use std::fmt::Display;
use core::time::Duration;

pub(crate) trait HttpRequest {
    fn new(timeout: Duration) -> Self;
    fn request(&self, method: HttpRequestMethod, url: &str) -> RequestBuilder;
}
#[derive(Clone)]
pub(crate) enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE
}

pub(crate) struct HttpClientRequest<T>{
    client: Option<T>,
    timeout: Duration

}

impl<T> HttpClientRequest<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn status_ok(&self, status: reqwest::StatusCode) -> bool {
        status.is_success()
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn status_ok(&self, status: u16) -> bool {
        (200..300).contains(&status)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpRequest for HttpClientRequest<Client> {
    fn new(timeout: Duration) -> Self {
        Self {client: Some(Client::builder()
            .user_agent(format!("blog-client/{}", env!("CARGO_PKG_VERSION")))
            .timeout(timeout)
            .build().expect("Failed to create client")),
            timeout
        }

    }

    fn request(&self, method: HttpRequestMethod, url: &str) -> RequestBuilder {
        let method_req = match method {
            HttpRequestMethod::GET => { Method::GET }
            HttpRequestMethod::POST => { Method::POST }
            HttpRequestMethod::PUT => { Method::PUT }
            HttpRequestMethod::DELETE => { Method::DELETE }
        };
        self.client.clone().expect("Not client").request(method_req, url)
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpRequest for HttpClientRequest<Request> {
    fn new(timeout: Duration) -> Self {
        Self {
            client: None,
            timeout
        }
    }

    fn request(&self, method: HttpRequestMethod, url: &str) -> RequestBuilder {
        let user_agent = format!("blog-client/{}", env!("CARGO_PKG_VERSION"));
         match method {
            HttpRequestMethod::GET => {
                Request::get(url).header(
                    "User-Agent",
                    user_agent.as_str(),
                )
            }
            HttpRequestMethod::POST => {
                Request::post(url).header(
                    "User-Agent",
                    user_agent.as_str(),
                )
            }
            HttpRequestMethod::PUT => {
                Request::put(url).header(
                    "User-Agent",
                    user_agent.as_str(),
                )
            }
             HttpRequestMethod::DELETE => {
                 Request::delete(url).header(
                     "User-Agent",
                     user_agent.as_str(),
                 )}
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub(crate) trait RequestBuilderExt {
    fn bearer_auth<T:Display>(self, token: T) -> Self;
    fn json_request<T: Serialize>(self, data: &T) -> Self;
}
#[cfg(target_arch = "wasm32")]
impl RequestBuilderExt for gloo_net::http::RequestBuilder {
    fn bearer_auth<T:Display>(mut self, token: T) -> Self {
        self = self.header("Authorization", &format!("Bearer {token}"));
        self
    }

    fn json_request<T: Serialize>(mut self, data: &T) -> Self {
        if let Ok(data) = serde_json::to_string(data) {
            self = self.header("Content-Type", "application/json");
            let req = self.body(data).expect("Failed to create request");
            let url = req.url().to_string();
            let method = req.method();
            self = gloo_net::http::RequestBuilder::new(&url).method(method);
        }
        self

    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) trait RequestBuilderExt {
    fn json_request<T: Serialize>(self, data: &T) -> Self;
}

#[cfg(not(target_arch = "wasm32"))]
impl RequestBuilderExt for reqwest::RequestBuilder {
    fn json_request<T: Serialize>(self, data: &T) -> Self {
        self.json(data)
    }
}
