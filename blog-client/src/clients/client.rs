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
/// Enum `Transport` represents the types of transport mechanisms that can be used for communication.
///
/// # Variants
///
/// - `Http(String)`
///   Represents an HTTP transport with the specified base URL as a `String`.
///
/// - `Grpc(String)` *(only available when the target architecture is not `wasm32`)*
///   Represents a gRPC transport with the specified address as a `String`.
///
/// # Notes
/// - The `Grpc` variant is conditionally compiled and will not be available when targeting `wasm32`.
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
///     let grpc_transport = Transport::Grpc(String::from("localhost:50051"));
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
/// A client for interacting with a blog API.
///
/// The `BlogClient` struct provides the necessary facilities to make requests to a blog back-end,
/// including support for HTTP and gRPC communications, token management, and thread-safe cloning.
///
/// # Fields
///
/// - `transport`: The transport mechanism used for API communication (e.g., HTTP, gRPC).
/// - `http_client`: An optional HTTP client instance wrapped in an `Arc`, enabling shared ownership
///    across threads. When `None`, HTTP communication is disabled.
/// - `grpc_client`: *(Only available on non-WASM targets)* An optional gRPC client instance wrapped
///    in an `Arc`. This allows shared ownership of the client in environments that support gRPC.
///    On WebAssembly (WASM) targets, this field is omitted to ensure compatibility.
/// - `token`: A thread-safe wrapper around the authentication token, stored as an `Option<String>`.
///    The token is shared using an `Arc<RwLock<...>>` to allow concurrent access and updates.
///
/// # Notes
///
/// This struct derives the `Clone` trait, enabling creation of multiple independent instances
/// of `BlogClient` that share the same underlying resources (e.g., clients and token).
///
/// The client adapts based on the target architecture:
/// - On platforms where gRPC is supported, the gRPC client can be utilized.
/// - On WebAssembly (WASM) targets, gRPC functionality is removed to optimize compatibility.
///
/// # Example
///
/// ```
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
    /// Creates and initializes a new instance of `BlogClient`.
    ///
    /// This asynchronous function configures the client based on the provided transport type
    /// (`Transport::Http` or `Transport::Grpc`) and sets up the necessary underlying clients
    /// for communication. It applies a timeout value for operations where applicable.
    ///
    /// # Parameters
    ///
    /// - `transport`: Specifies the type of transport to be used for communication. Supported
    ///   options are:
    ///   - `Transport::Http(base_url)`: Configures an HTTP client with the given `base_url`.
    ///   - `Transport::Grpc(addr)`: Configures a gRPC client to connect to the given `addr`.
    ///
    /// - `timeout`: A `Duration` value specifying the timeout for client operations.
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, BlogClientError>`:
    /// - `Ok(Self)`: On successful initialization of the client.
    /// - `Err(BlogClientError)`: If an error occurs during the setup process, such as failure
    ///   to initialize the HTTP client or gRPC client.
    ///
    /// # Platform-specific Behavior
    ///
    /// - When the target architecture is `wasm32`, only the HTTP client configuration (`Transport::Http`)
    ///   is supported. If `Transport::Grpc` is passed, it will result in a compile-time error as gRPC
    ///   is not available in `wasm32` builds.
    /// - For non-`wasm32` targets, both `Transport::Http` and `Transport::Grpc` are supported.
    ///
    /// # Examples
    ///
    /// Example usage with an HTTP transport:
    /// ```rust
    /// let transport = Transport::Http("https://api.example.com".to_string());
    /// let timeout = Duration::from_secs(30);
    /// let client = BlogClient::new(transport, timeout).await?;
    /// ```
    ///
    /// Example usage with a gRPC transport (non-`wasm32` targets only):
    /// ```rust
    /// let transport = Transport::Grpc("http://localhost:50051".to_string());
    /// let timeout = Duration::from_secs(30);
    /// let client = BlogClient::new(transport, timeout).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// - If the provided transport type fails to initialize the associated client (e.g., due to
    ///   invalid URL, unreachable server, or configuration errors), an error of type `BlogClientError`
    ///   will be returned.
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
    /// Creates a new instance of `BlogClient` configured to communicate over HTTP.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the HTTP server as a string or a type that can be converted into a string.
    /// * `timeout` - The duration after which HTTP requests will timeout.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the initialized `BlogClient` on success, or a `BlogClientError` on failure.
    ///
    /// # Example
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
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return a `BlogClientError` if there is an error during the initialization
    /// of the HTTP transport or client configuration.
    /// ```
    pub async fn http(
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, BlogClientError> {
        Self::new(Transport::Http(base_url.into()), timeout).await
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn grpc(addr: impl Into<String>, timeout: Duration) -> Result<Self, BlogClientError> {
        Self::new(Transport::Grpc(addr.into()), timeout).await
    }

    /// ```rust
    /// Sets the authentication token for the current instance and updates associated clients.
    ///
    /// This asynchronous function updates the internal token and propagates it to the associated
    /// HTTP and gRPC clients if they exist. Additionally, it saves the token persistently
    /// using `save_token` if the token is provided. Behavior may differ based on platform.
    ///
    /// # Parameters
    ///
    /// * `token` - An `Option<String>` representing the authentication token to be set.
    ///   If `None` is provided, the token is cleared.
    ///
    /// # Behavior
    ///
    /// 1. Updates the internal token by acquiring a write lock on the token storage.
    /// 2. If an HTTP client exists (`self.http_client`):
    ///    - It updates the HTTP client's token.
    ///    - If the token is not `None`, it attempts to save the token persistently using `save_token`.
    /// 3. If running on a platform other than WebAssembly (`not(target_arch = "wasm32")`),
    ///    and a gRPC client exists (`self.grpc_client`):
    ///    - It updates the gRPC client's token asynchronously.
    ///
    /// # Platform-Specific Notes
    ///
    /// * The gRPC client update is excluded on WebAssembly targets (`wasm32` architecture).
    ///
    /// # Example
    ///
    /// ```rust
    /// // Example usage of set_token.
    /// let client = MyClient::new();
    /// client.set_token(Some("my-auth-token".to_string())).await;
    /// ```
    ///
    /// # Errors
    ///
    /// * Saving the token with `save_token` might fail silently (`let _ = ...`),
    ///   but does not propagate the error to the caller.
    ///
    /// # Concurrency
    ///
    /// This method uses an `RwLock` for thread-safe operations on the token.
    ///
    /// # Async Behavior
    ///
    /// This function is asynchronous and should be awaited to ensure proper execution.
    /// ```
    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token.clone();
        if let Some(token) = token.clone() {
                let _ = self.save_token(&token);
        }
    }
    /// ```rust
    ///     /// Asynchronously retrieves the current token stored within the object.
    ///     ///
    ///     /// This function reads the token from the internal state, using an asynchronous
    ///     /// lock to ensure thread-safe access. The behavior is consistent regardless
    ///     /// of whether the code is executed in a WebAssembly (wasm32) target or a
    ///     /// non-WebAssembly target, as both configurations employ the same logic.
    ///     ///
    ///     /// # Returns
    ///     /// - `Option<String>`: Returns `Some(token)` if a token is present, or `None` if
    ///     ///   no token is stored.
    ///     ///
    ///     /// # Notes
    ///     /// - The function leverages asynchronous behavior, which makes it suitable for use
    ///     ///   in environments that support async runtimes.
    ///     /// - Internally uses a read lock to safely access the token.
    ///     ///
    ///     /// # Example
    ///     /// ```
    ///     /// let my_struct = MyStruct::new();
    ///     /// let token = my_struct.get_token().await;
    ///     /// if let Some(t) = token {
    ///     ///     println!("Token: {}", t);
    ///     /// } else {
    ///     ///     println!("No token available.");
    ///     /// }
    ///     /// ```
    /// ```
    pub async fn get_token(&self) -> Option<String> {
        if cfg!(target_arch = "wasm32") {
            self.token.read().await.clone()
        } else {
            self.token.read().await.clone()
        }
    }
    /// ```rust
    /// Registers a new user with the given username, email, and password.
    ///
    /// Depending on the configuration of `Self`, the registration process could use
    /// either an HTTP client or a gRPC client. The function is asynchronous and returns
    /// a `Response` upon successful registration or an `BlogClientError` if an error occurs.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice that holds the username for the new user.
    /// * `email` - A string slice that holds the email address for the new user.
    /// * `password` - A string slice that holds the password for the new user.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// - `Ok(Response)` on successful registration.
    /// - `Err(BlogClientError)` if no transport is configured or other errors occur.
    ///
    /// # Platform-specific Behavior
    ///
    /// * **WASM32 Target** - If compiled for WebAssembly (`wasm32`), this function also
    ///   saves the username and ID of the user into local storage via the `save_username`
    ///   method, provided the `id` is present in the response.
    /// * **Non-WASM32 Target** - On other targets, no additional storage-related operations
    ///   are performed.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following situations:
    /// - If neither an HTTP client nor a gRPC client is configured (`BlogClientError::NoTransportConfigured`).
    /// - If there are errors during the registration process, such as communication issues with the
    ///   server or invalid parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use your_crate::BlogClient;
    ///
    /// let client = BlogClient::new_with_http(); // Assuming an initialized client.
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
    /// Attempts to authenticate a user with the provided `username` and `password`.
    ///
    /// Depending on the platform and transport configuration, this function utilizes the
    /// appropriate client (HTTP or gRPC) to execute the login request. Upon successful
    /// login, this function may also store authentication tokens or persistent user
    /// information as needed.
    ///
    /// # Parameters
    ///
    /// - `username`:
    ///   The username of the user trying to log in. This should be a non-empty string.
    /// - `password`:
    ///   The password associated with the provided username. This should be a secure
    ///   string and should not be logged or stored in plaintext.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    /// - `Ok(Response)`: If the login request is successful. The `Response` contains
    ///   the token and optionally the user ID information.
    /// - `Err(BlogClientError)`: If an error occurs during the login process, such as
    ///   no transport being configured (`NoTransportConfigured`) or issues during the
    ///   client communication process.
    ///
    /// # Behavior
    ///
    /// - **HTTP Transport (WASM-32 target architecture):**
    ///   - Initiates an HTTP login request using the `http_client`.
    ///   - Stores the authentication token for future requests.
    ///   - If a user ID is returned in the response, it saves the username and ID
    ///     persistently via `save_username`.
    ///
    /// - **gRPC Transport (Non-WASM-32 architectures):**
    ///   - Initiates a gRPC login request using the `grpc_client`.
    ///   - Stores the authentication token for future requests.
    ///
    /// - **Fallback:**
    ///   - If no transport client (`http_client` or `grpc_client`) is configured,
    ///     the function returns `Err(B
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
    /// Creates a new blog post with the given title and content.
    ///
    /// This function interacts with the configured transport (HTTP or gRPC) to send
    /// a request to create a new blog post. Depending on the underlying platform and
    /// configuration, the transport mechanism may vary:
    ///
    /// - If the client is configured with an HTTP client (`http_client`), it will
    ///   use the HTTP transport to create the post.
    /// - If the client is configured with a gRPC client (`grpc_client`), and the
    ///   target architecture is not `wasm32`, it will use gRPC to create the post.
    /// - If no transport mechanism is configured, the function will return a
    ///   `BlogClientError::NoTransportConfigured` error.
    ///
    /// # Parameters
    ///
    /// - `title`: The title for the new post. A string slice representing the title text.
    /// - `content`: The content of the new post. A string slice representing the body text.
    ///
    /// # Returns
    ///
    /// An `async` function that returns a `Result`:
    /// - `Ok(Response)`: On successful creation of the blog post, it returns the response
    ///   from the transport layer containing details about the newly created post.
    /// - `Err(BlogClientError)`: If an error occurs during the process. Possible errors
    ///   include a lack of a configured transport mechanism or errors in the underlying
    ///   transport.
    ///
    /// # Errors
    ///
    /// - `BlogClientError::NoTransportConfigured`: Returned if neither an HTTP nor gRPC
    ///   client is configured.
    /// - Other variants of `BlogClientError` may occur depending on the implementation
    ///   of the transport mechanism used.
    ///
    /// # Platform-Specific Behavior
    ///
    /// - The gRPC transport (`grpc_client`) is not supported on the `wasm32` target architecture.
    ///
    /// # Example
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
    /// Retrieves a blog post by its unique identifier asynchronously.
    ///
    /// This function attempts to fetch a blog post using the available transport client:
    /// - If an HTTP client is configured, it will attempt to fetch the post via HTTP.
    /// - If a gRPC client is configured (and the target architecture is not WebAssembly),
    ///   it will attempt to fetch the post via gRPC.
    /// - If no transport client is configured, an error is returned.
    ///
    /// # Parameters
    /// - `id` (i64): The unique identifier of the blog post to retrieve.
    ///
    /// # Returns
    /// - `Result<Response, BlogClientError>`:
    ///   - `Ok(Response)`: The retrieved blog post as a response object.
    ///   - `Err(BlogClientError)`: An error indicating why the operation failed:
    ///       - `BlogClientError::NoTransportConfigured` if no transport client is configured.
    ///
    /// # Errors
    /// This function returns `BlogClientError::NoTransportConfigured` if neither an HTTP nor a gRPC
    /// client is available.
    ///
    /// # Notes
    /// - The function is asynchronous and must be awaited.
    /// - gRPC client support is disabled when the target architecture is WebAssembly (`wasm32`).
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
    /// Updates a blog post with the given `id`, updating its `title` and `content`.
    ///
    /// This function attempts to update a post via the configured client transport.
    /// If an HTTP client is available, the update is performed via HTTP. If a gRPC
    /// client is available (and not on wasm32 target), the update is performed via gRPC.
    /// If neither client is configured, an error is returned.
    ///
    /// # Parameters
    /// - `id`: The unique identifier of the post to be updated.
    /// - `title`: The new title for the post.
    /// - `content`: The new content for the post.
    ///
    /// # Returns
    /// - `Ok(Response)`: The response from the service after successfully updating the post.
    /// - `Err(BlogClientError::NoTransportConfigured)`: Returned if no suitable transport (HTTP or gRPC)
    ///   is configured for the operation.
    /// - `Err(BlogClientError)`: Other errors propagated from the underlying transport client.
    ///
    /// # Supported Platforms
    /// - Supports both HTTP and gRPC clients.
    /// - gRPC transport is not available for the WASM32 target.
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
    /// Deletes a blog post with the specified ID.
    ///
    /// This asynchronous function attempts to delete a blog post by leveraging one
    /// of the configured transport clients (`http_client` or `grpc_client`) available
    /// within the `Self` instance. The function prioritizes the HTTP client if both
    /// are configured. If no transport client is configured, the function returns an error.
    ///
    /// # Arguments
    ///
    /// * `id` - A 64-bit integer representing the ID of the blog post to be deleted.
    ///
    /// # Returns
    ///
    /// * `Ok(Response)` - If the deletion request is successfully processed by the client.
    /// * `Err(BlogClientError)` - If no transport client is configured or the client
    ///   returns an error during the deletion process.
    ///
    /// # Behavior
    ///
    /// - If an HTTP client is configured (`http_client: Some(client)`), it will be used to
    ///   execute the delete operation.
    /// - On non-WASM32 targets, if a gRPC client (`grpc_client: Some(client)`) is configured and
    ///   the HTTP client is not available, it will be used to execute the delete operation.
    /// - If no transport client is configured (`http_client` and `grpc_client` are both `None`),
    ///   the function returns `Err(BlogClientError::NoTransportConfigured)`.
    ///
    /// # Supported Platforms
    ///
    /// - The `grpc_client` transport is not available on WASM32 targets. When compiled for
    ///   such targets, only the HTTP transport is supported.
    ///
    /// # Errors
    ///
    /// The function returns a `BlogClientError` in the following cases:
    /// - `BlogClientError::NoTransportConfigured`: If neither `http_client` nor `grpc_client` are
    ///   available in the `Self` instance.
    /// - Other transport-layer errors originating from the `http_client` or `grpc_client`, which
    ///   will be propagated to the caller.
    ///
    /// # Examples
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
    /// Retrieves a list of blog posts with optional pagination parameters.
    ///
    /// This asynchronous function attempts to fetch posts by using the available client configuration.
    /// Depending on the target architecture, it utilizes either an HTTP client or gRPC client to retrieve
    /// the posts. If no transport mechanism is configured, it returns an error.
    ///
    /// # Parameters
    /// - `limit`: An optional maximum number of posts to retrieve. If `None`, no limit is applied.
    /// - `offset`: An optional starting position for retrieving posts. If `None`, no offset is applied.
    ///
    /// # Returns
    /// - `Ok(Response)`: A successful response containing the list of posts.
    /// - `Err(BlogClientError)`: An error if the posts cannot be retrieved or if no transport mechanism is configured.
    ///
    /// # Supported Clients
    /// - On `wasm32` targets: Uses the HTTP client (`http_client`).
    /// - On non-`wasm32` targets: If available, prefers the gRPC client (`grpc_client`) over the HTTP client.
    ///
    /// # Errors
    /// - `BlogClientError::NoTransportConfigured`: Returned if neither `http_client` nor `grpc_client` is configured.
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
    /// Provides access to the `Transport` instance associated with the current object.
    ///
    /// # Returns
    /// A reference to the `Transport` instance.
    ///
    /// # Examples
    /// ```rust
    /// let obj = MyStruct { transport: Transport::new() };
    /// let transport = obj.transport();
    /// // Use the transport instance as needed
    /// ```
    /// ```
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
    /// ```rust
    /// Asynchronously loads a token for authentication and sets it in the client.
    ///
    /// This method performs platform-specific token retrieval depending on the target architecture:
    ///
    /// - **Non-WASM platforms**:
    ///   - Attempts to read the token from a local file named `.blog_token` in the current directory.
    ///   - If the file exists, the token is loaded and set in the client.
    ///   - If the file does not exist, the method simply completes without taking further action.
    ///
    /// - **WASM (WebAssembly) platforms**:
    ///   - Retrieves the token from the web browser's `LocalStorage` under the key "blog_token".
    ///   - If the key exists, the token is loaded and set in the client.
    ///
    /// # Returns
    /// - `Ok(())` if the operation completes successfully.
    /// - `Err(BlogClientError)` if an error occurs during file reading (non-WASM) or token retrieval
    ///   (both platforms).
    ///
    /// # Errors
    /// - For non-WASM platforms, any I/O errors encountered while reading the token file will result
    ///   in an error.
    /// - For WASM platforms, errors related to `LocalStorage` access (e.g., storage key not found,
    ///   permission issues) will result in an error.
    ///
    /// # Platform Specific Notes
    /// - The method uses conditional compilation to execute platform-specific logic.
    /// - On non-WASM platforms, the file `.blog_token` must exist in the working directory to
    ///   successfully load a token.
    /// - On WASM platforms, `LocalStorage` must contain a valid token under the key "blog_token".
    ///
    /// # Example
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
    /// Saves the specified token for the blog client. This method handles saving the token
    /// differently depending on the target platform:
    ///
    /// - On non-WebAssembly (`wasm32`) platforms, the token is saved to a local file named `.blog_token`.
    /// - On WebAssembly (`wasm32`) platforms, the token is saved to the browser's `LocalStorage`.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice representing the token to be saved.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the token was saved successfully.
    /// * `Err(BlogClientError)` if an error occurred during the saving process.
    ///
    /// # Platform-Specific Behavior
    ///
    /// * **Non-WASM (`not(target_arch = "wasm32")`):**
    ///   - Uses the file system to write the token to a file named `.blog_token`.
    ///   - Requires sufficient permissions to write to the file system.
    /// * **WASM (`target_arch = "wasm32"`):**
    ///   - Saves the token in the browser's `LocalStorage` under the key `"blog_token"`.
    ///   - Also updates the internally stored token in the client instance.
    ///
    /// # Errors
    ///
    /// * Returns `BlogClientError` if there is an I/O failure in writing to the file (non-WASM),
    ///   or if there is an issue with accessing `LocalStorage` (WASM).
    ///
    /// # Example
    ///
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
    ///     /**
    ///      * Saves the username into the browser's local storage.
    ///      *
    ///      * This function is only available when compiled for the `wasm32` target architecture.
    ///      *
    ///      * # Parameters
    ///      * - `user` (`StorageUser`): The user object containing the username to be stored.
    ///      *
    ///      * # Returns
    ///      * - `Ok(())` if the username is successfully stored in the local storage.
    ///      * - `Err(BlogClientError)` if an error occurs while attempting to save the username.
    ///      *
    ///      * # Errors
    ///      * This function may return a `BlogClientError` if there is an issue setting the value
    ///      * in the browser's local storage.
    ///      *
    ///      * # Configuration
    ///      * This function is conditionally compiled and only available when targeting the
    ///      * WebAssembly `wasm32` architecture using the `#[cfg(target_arch = "wasm32")]` attribute.
    ///      *
    ///      * # Usage
    ///      * ```
    ///      * // Example usage assuming `user` is a valid `StorageUser` object
    ///      * match client.save_username(user) {
    ///      *     Ok(_) => println!("Username saved successfully!"),
    ///      *     Err(err) => eprintln!("Failed to save username: {}", err),
    ///      * }
    ///      * ```
    ///      */
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub fn save_username(&self, user: StorageUser) -> Result<(), BlogClientError> {
        LocalStorage::set("blog_username", &user)?;
        Ok(())
    }
    /// ```rust
    /// Clears the stored authentication token for the blog client.
    ///
    /// # Platform-specific Behavior
    ///
    /// This function works differently depending on the target architecture:
    ///
    /// - **Non-WebAssembly (`not(target_arch = "wasm32")`)**:
    ///   Deletes a local file named `.blog_token` from the filesystem if it exists.
    /// - **WebAssembly (`target_arch = "wasm32"`)**:
    ///   Deletes the `blog_token` and `blog_username` entries from the browser's local storage.
    ///
    /// # Errors
    ///
    /// - **Non-WebAssembly**:
    ///   Returns a `BlogClientError` if there is an issue removing the `.blog_token` file (e.g.,
    ///   file permissions, path issues).
    ///
    /// # Example
    /// ```rust
    /// let client = BlogClient::new();
    ///
    /// if let Err(e) = client.clear_token() {
    ///     eprintln!("Failed to clear token: {:?}", e);
    /// } else {
    ///     println!("Token cleared successfully.");
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// On WebAssembly (`wasm32`), this function does not return an error even if the specified
    /// local storage keys do not exist, as deletion is safe under such conditions.
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
    /// Retrieves the currently stored user information from the browser's local storage
    /// when running in a WebAssembly (WASM) environment.
    ///
    /// # Configuration
    /// This function is only compiled and available on the `wasm32` target architecture.
    /// It uses the `cfg` attribute to conditionally include this code for WebAssembly.
    ///
    /// # Returns
    /// - `Some(StorageUser)` if a user is stored in local storage under the key `"blog_username"`
    ///   and can be successfully deserialized.
    /// - `None` if the key `"blog_username"` does not exist or if deserialization fails.
    ///
    /// # Requirements
    /// This function depends on the presence of a valid WebAssembly environment with access
    /// to the `LocalStorage` API.
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
    /// # Errors
    /// This function does not explicitly raise errors; instead, it returns `None` when
    /// local storage access or deserialization fails.
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub fn get_user(&self) -> Option<StorageUser> {
        
        LocalStorage::get("blog_username").ok()
    }
}

/// ```rust
/// A builder for constructing instances of `BlogClient`.
///
/// The `BlogClientBuilder` struct provides a convenient way to configure and create
/// a `BlogClient` instance. It allows users to specify properties such as the transport mechanism,
/// request timeout, and authentication token.
///
/// # Fields
///
/// * `transport` - An optional transport layer (`Transport`) to be used for network communication. This could
///   define the protocol or infrastructure over which requests should be made (e.g., HTTP, HTTPS).
///
/// * `timeout` - An optional duration (`Duration`) specifying the maximum amount of time a request should take
///   before timing out. This is useful for ensuring responsiveness and preventing indefinite blocking.
///
/// * `token` - An optional authentication token (`String`) to be included with requests for secure communication
///   with the Blog API.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// let builder = BlogClientBuilder {
///     transport: Some(Transport::new()),
///     timeout: Some(Duration::from_secs(30)),
///     token: Some("my_secure_token".to_string()),
/// };
///
/// // After setting up the builder, you can construct a `BlogClient`
/// let client = builder.build();
/// ```
/// ```
pub struct BlogClientBuilder {
    transport: Option<Transport>,
    timeout: Option<Duration>,
    token: Option<String>,
}

impl BlogClientBuilder {
    /// ```rust
    /// Creates a new instance of the struct with default values.
    ///
    /// # Returns
    /// A new instance of the struct where:
    /// - `transport` is initialized to `None`,
    /// - `timeout` is initialized to `None`,
    /// - `token` is initialized to `None`.
    ///
    /// This function provides a convenient way to initialize the struct in its default state.
    /// ```
    pub fn new() -> Self {
        Self {
            transport: None,
            timeout: None,
            token: None,
        }
    }
    /// ```rust
    /// Sets the transport method for the object and returns the updated instance.
    ///
    /// This method allows you to specify a `Transport` instance, which will be used
    /// by the object. The provided `Transport` is stored in an internal optional field.
    ///
    /// # Parameters
    /// - `transport`: The `Transport` instance to set for the object.
    ///
    /// # Returns
    /// Returns the updated instance of the object with the specified `transport` set.
    ///
    /// # Example
    /// ```rust
    /// let obj = Object::new().transport(transport_instance);
    /// ```
    /// ```
    pub fn transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }
    /// ```rust
    ///     ///
    ///     /// Configures the client to use HTTP transport with the specified base URL.
    ///     ///
    ///     /// This method sets up the HTTP transport layer by initializing it with the
    ///     /// provided base URL. The base URL is converted into a `String` and then passed
    ///     /// to the HTTP transport layer.
    ///     ///
    ///     /// # Parameters
    ///     ///
    ///     /// * `base_url` - A type that can be converted into a `String`. Represents the
    ///     ///   base URL of the HTTP endpoint that the client will communicate with.
    ///     ///
    ///     /// # Returns
    ///     ///
    ///     /// Returns an updated instance of `Self` with the HTTP transport configured.
    ///     ///
    ///     /// # Examples
    ///     ///
    ///     /// ```
    ///     /// let client = Client::new().http("https://api.example.com");
    ///     /// ```
    ///     ///
    ///     /// In this example, the client is configured to use the HTTP transport with
    ///     /// "https://api.example.com" as the base URL.
    ///     ///
    /// ```
    pub fn http(mut self, base_url: impl Into<String>) -> Self {
        self.transport = Some(Transport::Http(base_url.into()));
        self
    }

    /// ```rust
    ///     /**
    ///      * Configures the transport layer to use gRPC with the specified address.
    ///      *
    ///      * This method is only available when the target architecture is not WebAssembly (`wasm32`),
    ///      * as gRPC is not supported in that environment.
    ///      *
    ///      * # Parameters
    ///      * - `addr`: A value that can be converted into a `String` representing the address
    ///      *           of the gRPC endpoint.
    ///      *
    ///      * # Returns
    ///      * - `Self`: The instance of the builder with the updated transport configuration.
    ///      *
    ///      * # Example
    ///      * ```
    ///      * let builder = Builder::new().grpc("http://localhost:50051");
    ///      * ```
    ///      *
    ///      * # Configuration
    ///      * The method utilizes the conditional compilation attribute `#[cfg(not(target_arch = "wasm32"))]`,
    ///      * ensuring that it is excluded from builds targeting WebAssembly.
    ///      */
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn grpc(mut self, addr: impl Into<String>) -> Self {
        self.transport = Some(Transport::Grpc(addr.into()));
        self
    }
    /// ```rust
    /// Sets the token for the current instance.
    ///
    /// This method assigns a token to the instance by converting the given input
    /// into a `String` and storing it in the `token` field. It returns the updated
    /// instance, allowing for method chaining.
    ///
    /// # Parameters
    /// - `token`: Any type that can be converted into a `String` (e.g., `&str` or `String`).
    ///
    /// # Returns
    /// - `Self`: The updated instance with the token set.
    ///
    /// # Example
    /// ```
    /// let instance = MyStruct::new()
    ///     .token("my-secret-token");
    /// ```
    /// ```
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// ```rust
    /// Builds and initializes a `BlogClient` instance asynchronously.
    ///
    /// # Returns
    /// - `Ok(BlogClient)` if the initialization completes successfully.
    /// - `Err(BlogClientError)` if there is a configuration error or if the `BlogClient` cannot be created.
    ///
    /// # Errors
    /// - Returns `BlogClientError::Config` if the required `transport` is not specified in the configuration.
    /// - Returns an error if the underlying client initialization or token setup fails.
    ///
    /// # Default Values
    /// - If `timeout` is not provided, it defaults to 30 seconds.
    ///
    /// # Token Handling
    /// - If an optional `token` is provided, it will be set on the created `BlogClient` instance.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// use your_crate::{BlogClientBuilder, BlogClientError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BlogClientError> {
    ///    let client = BlogClientBuilder::default()
    ///        .transport(my_transport)
    ///        .timeout(Duration::from_secs(10))
    ///        .token(Some("my-token".to_string()))
    ///        .build()
    ///        .await?;
    ///
    ///    // Use the `client` for further operations
    ///    Ok(())
    /// }
    /// ```
    /// ```
    pub async fn build(self) -> Result<BlogClient, BlogClientError> {
        let transport = self
            .transport
            .ok_or_else(|| BlogClientError::Config("Transport not specified".to_string()))?;
        let timeout = self.timeout.unwrap_or_else(|| Duration::from_secs(30));
        let client = BlogClient::new(transport, timeout).await?;

        if let Some(token) = self.token {
            client.set_token(Some(token)).await;
        }

        Ok(client)
    }
}

impl Default for BlogClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
