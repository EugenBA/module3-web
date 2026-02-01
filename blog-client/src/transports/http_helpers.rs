use std::error::Error;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Method, RequestBuilder, Client, Response};
#[cfg(target_arch = "wasm32")]
use gloo_net::http::{Request, RequestBuilder, Response};
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use std::fmt::Display;
#[cfg(target_arch = "wasm32")]
use log::{info, error, warn, debug, trace};
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

pub(crate) struct HttpClientRequestBuilder {
    #[cfg(target_arch = "wasm32")]
    request: Request,
    #[cfg(not(target_arch = "wasm32"))]
    request: RequestBuilder
}

impl HttpClientRequestBuilder{
    pub async fn send(&self) -> Response {
        self.send().await?
    }
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
        wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
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
    fn json_request<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder;
}
#[cfg(target_arch = "wasm32")]
impl RequestBuilderExt for gloo_net::http::RequestBuilder {
    fn bearer_auth<T:Display>(mut self, token: T) -> Self {
        self = self.header("Authorization", &format!("Bearer {token}"));
        self
    }

    fn json_request<T: Serialize>(mut self, data: &T) -> HttpClientRequestBuilder {
        trace!("Json create: {}", data);
        let user_agent = format!("blog-client/{}", env!("CARGO_PKG_VERSION"));
        self = self.header("Content-Type", "application/json")
                       .header("User-Agent", user_agent.as_str());
        trace!("RB: {:?}", self);
        let req = self.json(data).except("error blog client json data");
        HttpClientRequestBuilder{
            request: self
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) trait RequestBuilderExt {
    fn json_request<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder;
}

#[cfg(not(target_arch = "wasm32"))]
impl RequestBuilderExt for RequestBuilder {
    fn json_request<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder {
        HttpClientRequestBuilder{
            request: self.json(data)
        }
    }
}
