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
use std::fmt;
use crate::error::BlogClientError;

pub(crate) trait HttpRequest {
    fn new(timeout: Duration) -> Self;
    fn request(&self, method: HttpRequestMethod, url: &str) -> HttpClientRequestBuilder;
}

pub(crate) trait HttpBuilder{
    fn json<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder;
    fn bearer_auth<T:fmt::Display>(self, token: T) -> HttpClientRequestBuilder;
    async fn send(self) -> Result<Response, BlogClientError>;

}
#[derive(Clone)]
pub(crate) enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE
}

pub(crate) struct HttpClientRequest{
    #[cfg(target_arch = "wasm32")]
    client: Request,
    #[cfg(not(target_arch = "wasm32"))]
    client: Client,
    timeout: Duration

}

pub(crate) struct HttpClientRequestBuilder {
    #[cfg(target_arch = "wasm32")]
    request: Request,
    #[cfg(not(target_arch = "wasm32"))]
    request: RequestBuilder
}


#[cfg(not(target_arch = "wasm32"))]
impl HttpBuilder for HttpClientRequestBuilder{
    fn json<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder {
        let request = self.request.json(data);
        HttpClientRequestBuilder{
            request
        }
    }

    fn bearer_auth<T>(self, token: T) -> HttpClientRequestBuilder
        where
        T: fmt::Display,
        {
        Self{
            request: self.request.bearer_auth(token),
        }
    }

    async fn send(self) -> Result<Response, BlogClientError> {
        Ok(self.request.send().await?)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpRequest for HttpClientRequest {
    fn new(timeout: Duration) -> Self {
        Self {client: Client::builder()
            .user_agent(format!("blog-client/{}", env!("CARGO_PKG_VERSION")))
            .timeout(timeout)
            .build().expect("Failed to create client"),
            timeout
        }

    }
    fn request(&self, method: HttpRequestMethod, url: &str) -> HttpClientRequestBuilder {
        let method_req = match method {
            HttpRequestMethod::GET => { Method::GET }
            HttpRequestMethod::POST => { Method::POST }
            HttpRequestMethod::PUT => { Method::PUT }
            HttpRequestMethod::DELETE => { Method::DELETE }
        };
        let request = self.client.clone().request(method_req, url);
        HttpClientRequestBuilder{
            request,
        }
    }
}


#[cfg(target_arch = "wasm32")]
impl HttpRequest for HttpClientRequest{
    fn new(timeout: Duration) -> Self {
        Self {
            client: Request::default(),
            timeout
        }
    }
    fn request(&self, method: HttpRequestMethod, url: &str) -> HttpClientRequestBuilder {
        let user_agent = format!("blog-client/{}", env!("CARGO_PKG_VERSION"));
        let request = match method {
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
        };
        HttpClientRequestBuilder{
            request
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpBuilder for HttpClientRequestBuilder{
    fn json<T: Serialize>(self, data: &T) -> HttpClientRequestBuilder {
        let user_agent = format!("blog-client/{}", env!("CARGO_PKG_VERSION"));
        self = self.request.header("User-Agent", user_agent.as_str());
        let request = self.json(data).except("error blog client json data");
        HttpClientRequestBuilder{
            request
        }
    }

    fn bearer_auth<T>(self, token: T) -> HttpClientRequestBuilder
    where
        T: fmt::Display,
    {
        let request = self.request.header("Authorization", &format!("Bearer {token}"));
        Self{
            request
        }
    }

    async fn send(self) -> Result<Response, BlogClientError> {
        Ok(self.request.send().await?)
    }
}


impl HttpClientRequest {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn status_ok(&self, status: reqwest::StatusCode) -> bool {
        status.is_success()
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn status_ok(&self, status: u16) -> bool {
        (200..300).contains(&status)
    }
}
