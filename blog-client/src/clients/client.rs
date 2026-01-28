use std::fs;
use std::path::Path;
use crate::models::models::{Response, Post};
use crate::{error::BlogClientError, grpc_client::GrpcClient, http_client::HttpClient};
use std::sync::Arc;
use tokio::sync::RwLock;

/// ```rust
/// Represents a transport mechanism for communication, which can be either HTTP or gRPC.
///
/// # Variants
///
/// * `Http(String)`
///   Represents an HTTP transport with the specified base URL.
///
/// * `Grpc(String)`
///   Represents a gRPC transport with the specified address.
///
/// # Examples
///
/// ```rust
/// use your_crate::Transport;
///
/// let http_transport = Transport::Http(String::from("http://example.com"));
/// let grpc_transport = Transport::Grpc(String::from("127.0.0.1:50051"));
///
/// println!("{:?}", http_transport);
/// println!("{:?}", grpc_transport);
/// ```
/// #[
#[derive(Debug, Clone)]
pub enum Transport {
    /// HTTP транспорт с указанием базового URL
    Http(String),
    /// gRPC транспорт с указанием адреса
    Grpc(String),
}

impl Transport {
    /// Создает HTTP транспорт с указанием базового URL
    pub fn http(base_url: impl Into<String>) -> Self {
        Self::Http(base_url.into())
    }

    /// Создает gRPC транспорт с указанием адреса
    pub fn grpc(addr: impl Into<String>) -> Self {
        Self::Grpc(addr.into())
    }
}

/// ```rust
/// Represents a client for interacting with the Blog service.
///
/// The `BlogClient` struct provides functionality to communicate with the Blog service
/// using various transport mechanisms, such as HTTP or gRPC. It supports managing
/// authentication tokens and handles the underlying transport layer for executing requests.
///
/// # Fields
///
/// - `transport`: Specifies the transport mechanism (e.g., HTTP or gRPC) used by the client to interact
///   with the Blog service.
/// - `http_client`: An optional reference-counted `Arc` pointer to an `HttpClient`, which is used for
///   executing HTTP-based requests if the transport is set to HTTP.
/// - `grpc_client`: An optional reference-counted `Arc` pointer to a `GrpcClient`, utilized for enabling
///   gRPC-based communication if the transport mode is gRPC.
/// - `token`: A thread-safe, reference-counted `Arc` wrapper around a read-write lock
///   (`RwLock`) containing an optional authentication token, which can be updated and accessed safely
///   across multiple threads.
///
/// # Derives
///
/// - `Clone`: The `BlogClient` struct can be cloned, creating a copy of the client while sharing
///   references to internal data such as the `http_client`, `grpc_client`, and `token`.
///
/// # Example
///
/// ```rust
/// # use std::sync::{Arc, RwLock};
/// # use crate::{Transport, HttpClient, GrpcClient, BlogClient};
/// // Create a new BlogClient for HTTP transport.
/// let http_client = Arc::new(HttpClient::new());
/// let blog_client = BlogClient {
///     transport: Transport::Http,
///     http_client: Some(http_client),
///     grpc_client: None,
///     token: Arc::new(RwLock::new(None)),
/// };
///
/// // Update the authentication token.
/// {
///     let mut token = blog_client.token.write().unwrap();
///     *token = Some("my_auth_token".to_string());
/// }
///
/// // Access the token.
/// {
///     let token = blog_client.token.read().unwrap();
///     println!("Current token: {:?}", *token);
/// }
/// ```
/// ```
#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<HttpClient>>,
    grpc_client: Option<Arc<GrpcClient>>,
    token: Arc<RwLock<Option<String>>>,
}

impl BlogClient {
    /// ```rust
    /// Creates a new instance of the `BlogClient` asynchronously, initializing it based on the provided transport type.
    ///
    /// # Parameters
    /// - `transport`: The transport mechanism to be used by the client. Can either be:
    ///     - `Transport::Http(base_url)`: An HTTP transport initialized with a base URL.
    ///     - `Transport::Grpc(addr)`: A gRPC transport initialized with an address.
    ///
    /// # Returns
    /// - `Ok(Self)`: A new instance of `BlogClient` on successful initialization.
    /// - `Err(BlogClientError)`: If any errors occur during initialization of the HTTP or gRPC client.
    ///
    /// # Behavior
    /// - For `Transport::Http`:
    ///     - A new `HttpClient` is created using the provided `base_url`.
    ///     - The resulting client instance has `http_client` set, while `grpc_client` is `None`.
    /// - For `Transport::Grpc`:
    ///     - A new `GrpcClient` is created using the provided `addr`.
    ///     - The resulting client instance has `grpc_client` set, while `http_client` is `None`.
    ///
    /// # Fields
    /// - `http_client`: Optionally holds the HTTP client if the transport type is HTTP.
    /// - `grpc_client`: Optionally holds the gRPC client if the transport type is gRPC.
    /// - `token`: An `Arc<RwLock<Option<Token>>>` used for handling authorization tokens.
    ///
    /// # Errors
    /// - Returns `BlogClientError` if:
    ///   - The `HttpClient` initialization fails when using `Transport::Http`.
    ///   - The `GrpcClient` initialization fails when using `Transport::Grpc`.
    ///
    /// # Example
    /// ```rust
    /// use blog_client::{Transport, BlogClient};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let http_transport = Transport::Http("https://example.com".to_string());
    ///     let client = BlogClient::new(http_transport).await;
    ///     match client {
    ///         Ok(client) => println!("Client initialized successfully!"),
    ///         Err(e) => eprintln!("Failed to initialize client: {}", e),
    ///     }
    /// }
    /// ```
    /// ```
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        match &transport {
            Transport::Http(base_url) => {
                let http_client = HttpClient::new(base_url).await?;
                Ok(Self {
                    transport,
                    http_client: Some(Arc::new(http_client)),
                    grpc_client: None,
                    token: Arc::new(RwLock::new(None)),
                })
            }
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
    /// Creates a new instance of the `BlogClient` using the HTTP transport.
    ///
    /// # Parameters
    /// - `base_url`: A value that can be converted into a `String`, representing the base URL for the HTTP client.
    ///
    /// # Returns
    /// An asynchronous `Result` containing:
    /// - `Self`: An instance of the `BlogClient` if the client initialization succeeds.
    /// - `BlogClientError`: An error if the initialization fails.
    ///
    /// # Examples
    /// ```
    /// let client = BlogClient::http("https://api.example.com").await?;
    /// ```
    ///
    /// # Errors
    /// Returns a `BlogClientError` if the client fails to initialize.
    ///
    /// This function internally calls `Self::new()` with a `Transport::Http` transport type.
    /// ```
    pub async fn http(base_url: impl Into<String>) -> Result<Self, BlogClientError> {
        Self::new(Transport::Http(base_url.into())).await
    }

    /// ```rust
    ///     /// Creates a new instance of the `BlogClient` using gRPC as the transport layer.
    ///     ///
    ///     /// # Parameters
    ///     /// - `addr`: An address that implements `Into<String>`, representing the gRPC server's endpoint.
    ///     ///
    ///     /// # Returns
    ///     /// A `Result` containing:
    ///     /// - `Ok(Self)`: A successfully created instance of `BlogClient`.
    ///     /// - `Err(BlogClientError)`: An error encountered during the client initialization.
    ///     ///
    ///     /// # Example
    ///     /// ```rust
    ///     /// let client = BlogClient::grpc("http://localhost:50051").await?;
    ///     /// ```
    ///     ///
    ///     /// # Errors
    ///     /// Returns a `BlogClientError` if the client fails to initialize for any reason.
    ///     ///
    ///     /// # Async
    ///     /// This function is asynchronous and should be awaited.
    /// ```
    pub async fn grpc(addr: impl Into<String>) -> Result<Self, BlogClientError> {
        Self::new(Transport::Grpc(addr.into())).await
    }

    /// ```rust
    ///     /// Asynchronously updates the token for the current instance and propagates it
    ///     /// to associated HTTP and gRPC clients if they exist.
    ///     ///
    ///     /// # Parameters
    ///     /// - `token`: An `Option<String>` representing the new token to be set.
    ///     ///   If `None` is provided, the token will be cleared.
    ///     ///
    ///     /// # Behavior
    ///     /// - Updates the internal token by acquiring a write lock.
    ///     /// - If an HTTP client is associated with the instance, its token will also be updated asynchronously.
    ///     /// - Similarly, if a gRPC client is associated with the instance, its token will also be updated asynchronously.
    ///     ///
    ///     /// # Example
    ///     /// ```
    ///     /// instance.set_token(Some("new_token".to_string())).await;
    ///     /// instance.set_token(None).await; // Clears the token.
    ///     /// ```
    ///     ///
    ///     /// # Concurrency
    ///     /// This function uses asynchronous locks to safely update the token across multiple contexts.
    /// ```
    ///
    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token.clone();

        if let Some(http_client) = &self.http_client {
            http_client.set_token(token.clone()).await;
        }
        if let Some(grpc_client) = &self.grpc_client {
            grpc_client.set_token(token).await;
        }
    }

    /// ```rust
    /// Asynchronously retrieves a token stored within the instance.
    ///
    /// This function acquires a read lock on the `token` field, which is an
    /// asynchronous operation, and returns a cloned optional string containing
    /// the token value.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - If a token is present, it returns `Some(String)`
    /// containing the token. If no token is available, it returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// let token = instance.get_token().await;
    /// if let Some(t) = token {
    ///     println!("Token: {}", t);
    /// } else {
    ///     println!("No token available.");
    /// }
    /// # }
    /// ```
    ///
    /// # Notes
    /// This function is designed to be non-blocking and works in an asynchronous
    /// context. The use of a `.read().await` lock ensures safe concurrent access
    /// to the `token`.
    ///
    /// ```
    pub async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    /// ```rust
    /// Registers a new user by sending their credentials to the server and updates the client's token.
    ///
    /// # Parameters
    /// - `username`: A string slice holding the username of the user to register.
    /// - `email`: A string slice holding the email address of the user to register.
    /// - `password`: A string slice holding the password of the user to register.
    ///
    /// # Returns
    /// - `Ok(AuthResponse)`: On successful registration, returns an `AuthResponse` containing the server's response,
    ///   including the authentication token.
    /// - `Err(BlogClientError)`: Returns a `BlogClientError` if an error occurs during the registration process:
    ///    - `NoTransportConfigured`: The client does not have an HTTP or gRPC transport configured.
    ///    - Other transport-related or request-related errors.
    ///
    /// # Behavior
    /// - If the client is configured with an HTTP transport (`http_client`), it sends the registration request via HTTP.
    /// - If the client is configured with a gRPC transport (`grpc_client`), it sends the registration request via gRPC.
    /// - After a successful response, the authentication token is set for the client using the `set_token` method.
    /// - If neither `http_client` nor `grpc_client` is configured, the method will return a `NoTransportConfigured` error.
    ///
    /// # Examples
    /// ```rust
    /// let client = BlogClient::new_with_http(http_client);
    /// let result = client.register("username123", "user@example.com", "securepassword").await;
    /// match result {
    ///     Ok(auth_response) => println!("Registered successfully! Token: {}", auth_response.token),
    ///     Err(e) => eprintln!("Failed to register: {}", e),
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
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.register(username, email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```rust
    /// Asynchronously logs in a user using the provided credentials (username, email, and password).
    ///
    /// This method attempts to perform the login operation based on the underlying transport mechanism set up
    /// in the `BlogClient` instance. It supports HTTP and gRPC clients. If no transport is configured, an error
    /// is returned.
    ///
    /// # Arguments
    ///
    /// * `username` - A string reference representing the username of the user attempting to log in.
    /// * `email` - A string reference representing the email of the user attempting to log in.
    /// * `password` - A string reference representing the password of the user attempting to log in.
    ///
    /// # Returns
    ///
    /// On success, returns a `Result` containing an `AuthResponse`, which includes the authentication token
    /// and other metadata. The token is stored internally after a successful login.
    /// On failure, returns a `BlogClientError`, which could indicate either a configuration issue (e.g., no transport
    /// configured) or a failure in the login process itself.
    ///
    /// # Errors
    ///
    /// This method returns the following errors:
    ///
    /// * `BlogClientError::NoTransportConfigured` - Indicates no HTTP or gRPC client was configured in the `BlogClient` object.
    /// * Other `BlogClientError` variants as returned from the underlying client during the login process.
    ///
    /// # Example
    ///
    /// ```rust
    /// use blog_client::BlogClient;
    ///
    /// async fn run_login(client: BlogClient) -> Result<(), blog_client::BlogClientError> {
    ///     let username = "user123";
    ///     let email = "user123@example.com";
    ///     let password = "password123";
    ///
    ///     match client.login(username, email, password).await {
    ///         Ok(auth_response) => {
    ///             println!("Successfully logged in! Token: {}", auth_response.token);
    ///             Ok(())
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Failed to log in: {:?}", err);
    ///             Err(err)
    ///         }
    ///     }
    /// }
    /// ```
    /// ```
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```
    /// Creates a new blog post with the given title and content.
    ///
    /// This asynchronous function attempts to create a blog post using the available
    /// transport client (either HTTP or gRPC). If neither client is configured, it
    /// will return an error.
    ///
    /// # Parameters
    /// - `title`: A string slice representing the title of the post.
    /// - `content`: A string slice representing the content of the post.
    ///
    /// # Returns
    /// - `Ok(Post)`: A `Post` object representing the newly created post.
    /// - `Err(BlogClientError)`: An error if the operation fails, such as when there
    ///   is no transport client configured or the client encounters an issue.
    ///
    /// # Errors
    /// This function will return:
    /// - `BlogClientError::NoTransportConfigured` if neither HTTP nor gRPC client is available.
    /// - Other variants of `BlogClientError` depending on the transport client's implementation.
    ///
    /// # Usage
    /// ```rust
    /// let result = blog_client.create_post("My Title", "My Content").await;
    /// match result {
    ///     Ok(post) => println!("Post created: {:?}", post),
    ///     Err(e) => eprintln!("Failed to create post: {:?}", e),
    /// }
    /// ```
    /// ```
    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.create_post(title, content).await,
            Self {
                grpc_client: Some(client),
                ..
            } => client.create_post(title, content).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```
    /// Asynchronously retrieves a blog post by its ID using the available transport mechanism.
    ///
    /// This method attempts to fetch a blog post, utilizing either an HTTP or gRPC client.
    /// If no transport mechanism is configured, it returns an error.
    ///
    /// # Parameters
    /// - `id`: The unique identifier of the blog post to retrieve.
    ///
    /// # Returns
    /// - `Ok(Post)`: The blog post retrieved successfully.
    /// - `Err(BlogClientError)`: An error occurred during the retrieval, such as no transport being configured.
    ///
    /// # Errors
    /// - `BlogClientError::NoTransportConfigured`: Returned if neither HTTP nor gRPC client is configured for the instance.
    ///
    /// # Example
    /// ```rust
    /// let blog_client = BlogClient::new_with_http(http_client);
    /// match blog_client.get_post(42).await {
    ///     Ok(post) => println!("Retrieved post: {:?}", post),
    ///     Err(err) => eprintln!("Failed to retrieve post: {:?}", err),
    /// }
    /// ```
    ///
    /// # Notes
    /// - This function relies on the availability of a working transport mechanism (either HTTP or gRPC).
    ///   Ensure that the `BlogClient` is initialized with the appropriate transport client.
    /// ```
    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_post(id).await,
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```rust
    /// Updates an existing blog post by its unique identifier.
    ///
    /// # Parameters
    /// - `id`: The unique identifier of the post to be updated.
    /// - `title`: The new title to be assigned to the post.
    /// - `content`: The new content to be assigned to the post.
    ///
    /// # Returns
    /// - `Ok(Post)`: The updated blog post upon successful completion.
    /// - `Err(BlogClientError)`: An error if the update operation fails. Possible
    ///   errors include:
    ///   - `NoTransportConfigured`: No communication transport (HTTP or gRPC) is configured.
    ///
    /// # Behavior
    /// - If an HTTP client is configured, the update operation is performed
    ///   using the HTTP client.
    /// - If a gRPC client is configured, the update operation is performed
    ///   using the gRPC client.
    /// - If neither client is configured, the function will return a
    ///   `NoTransportConfigured` error.
    ///
    /// # Examples
    /// ```rust
    /// let updated_post = blog_client.update_post(42, "Updated Title", "Updated Content").await;
    /// match updated_post {
    ///     Ok(post) => println!("Post updated successfully: {:?}", post),
    ///     Err(e) => eprintln!("Failed to update post: {:?}", e),
    /// }
    /// ```
    /// ```
    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.update_post(id, title, content).await,
            Self {
                grpc_client: Some(client),
                ..
            } => client.update_post(id, title, content).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```rust
    /// Deletes a blog post by its unique identifier.
    ///
    /// This asynchronous function attempts to delete a blog post with the specified `id`
    /// by utilizing the available transport layer (either HTTP or gRPC).
    ///
    /// # Parameters
    /// - `id`: The unique identifier of the blog post to delete.
    ///
    /// # Returns
    /// - `Ok(())` if the blog post is successfully deleted.
    /// - `Err(BlogClientError)` if an error occurs during the operation:
    ///   - `BlogClientError::NoTransportConfigured`: If neither an HTTP nor a gRPC client
    ///     is configured on the instance.
    ///   - Other variants of `BlogClientError` if the specific transport layer encounters an issue.
    ///
    /// # Usage Example
    /// ```rust
    /// let client = BlogClient::new_with_http(http_client_instance);
    /// match client.delete_post(1234).await {
    ///     Ok(_) => println!("Post deleted successfully."),
    ///     Err(e) => eprintln!("Failed to delete post: {:?}", e),
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function requires either an `http_client` or a `grpc_client` to be configured
    ///   on the instance; otherwise, it will return an error.
    /// - The specific behavior, such as whether the deletion is idempotent, depends on the
    ///   implementation of the underlying clients.
    /// ```
    pub async fn delete_post(&self, id: i64) -> Result<(), BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.delete_post(id).await,
            Self {
                grpc_client: Some(client),
                ..
            } => client.delete_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```rust
    /// Retrieves a list of blog posts with optional pagination.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional parameter to specify the maximum number of posts to retrieve.
    /// * `offset` - An optional parameter to specify the starting point for the list of posts.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` containing:
    /// * `Ok(Vec<Post>)` - A vector of `Post` objects upon success.
    /// * `Err(BlogClientError)` - An error if retrieval fails or if no transport client is configured.
    ///
    /// # Behavior
    ///
    /// This method checks whether the `http_client` or `grpc_client` is available in the instance
    /// and delegates the request to the appropriate client.
    /// * If an `http_client` is present, it uses the HTTP protocol to fetch posts.
    /// * If a `grpc_client` is present, it utilizes gRPC to fetch posts.
    /// * If neither client is configured, it returns a `BlogClientError::NoTransportConfigured`.
    ///
    /// # Errors
    ///
    /// Returns `BlogClientError::NoTransportConfigured` if neither an `http_client` nor
    /// a `grpc_client` is configured in the struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// let blog_client = BlogClient::new_with_http_client(http_client_instance);
    /// let posts = blog_client.list_posts(Some(10), Some(0)).await?;
    /// for post in posts {
    ///     println!("Post Title: {}", post.title);
    /// }
    /// ```
    /// ```
    pub async fn list_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Post>, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// ```rust
    /// Returns a reference to the `Transport` instance associated with the current object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let transport = my_object.transport();
    /// // Use `transport` as needed
    /// ```
    ///
    /// # Returns
    /// A reference to the `Transport` instance.
    ///
    /// # Notes
    /// - The returned reference allows read-only access to the `Transport`.
    /// - Useful for retrieving the `Transport` for further operations.
    ///
    /// # Safety
    /// This method is safe as it only returns an immutable reference.
    /// ```
    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    pub async fn load_token(&self) -> Result<(), BlogClientError> {
        let token_file = ".blog_token";
        if Path::new(token_file).exists() {
            let token=fs::read_to_string(token_file).ok();
            self.set_token(token).await;
        } else {
            return Err(BlogClientError::Unauthorized("Not load token".to_string()));
        }
        Ok(())
    }
    pub fn save_token(&self, token: &str) -> Result<(), BlogClientError> {
        fs::write(".blog_token", token)?;
        Ok(())
    }

}

/// ```rust
/// A builder for configuring and creating instances of `BlogClient`.
///
/// The `BlogClientBuilder` allows for the customized construction of a `BlogClient` by
/// setting various options such as the transport mechanism and authentication token.
/// This builder provides a flexible way to initialize a `BlogClient` with the desired
/// configurations before usage.
///
/// # Fields
///
/// * `transport` - An optional transport mechanism for handling requests.
///   If not provided, a default transport will need to be set or assumed during `BlogClient` creation.
/// * `token` - An optional authentication token used for authorized requests.
///   If unset, requests will likely be made without authentication.
///
/// # Example
/// ```rust
/// let builder = BlogClientBuilder {
///     transport: Some(Transport::new()),
///     token: Some("my-auth-token".to_string()),
/// };
/// let client = builder.build();
/// ```
///
/// # Notes
/// Typically, after setting the desired fields on `BlogClientBuilder`, the user would call a
/// `build` or equivalent method (implementation not shown) to create a `BlogClient` instance.
///
/// This struct is particularly useful when dealing with configurable or optional fields
/// where defaults may not always suit every use case.
/// ```
pub struct BlogClientBuilder {
    transport: Option<Transport>,
    token: Option<String>,
}

impl BlogClientBuilder {
    /// Создает новый билдер
    pub fn new() -> Self {
        Self {
            transport: None,
            token: None,
        }
    }

    /// ```rust
    /// Sets the transport configuration for the current instance.
    ///
    /// This method allows a transport mechanism to be specified for the object.
    /// The provided `Transport` instance will be stored within the object.
    /// The method consumes the current instance, applies the given transport,
    /// and returns the updated instance.
    ///
    /// # Parameters
    /// - `transport`: The transport configuration to be applied. This value will
    ///   be wrapped in an `Option` and stored in the `transport` field.
    ///
    /// # Returns
    /// - `Self`: The updated instance with the newly assigned transport.
    ///
    /// # Example
    /// ```
    /// let my_instance = MyStruct::new().transport(my_transport);
    /// ```
    ///
    /// This example demonstrates creating a new instance of `MyStruct` and
    /// assigning it a transport configuration.
    /// ```
    pub fn transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }

    /// ```rust
    /// Sets the transport mechanism to HTTP with the specified base URL.
    ///
    /// This method configures the `Transport` to use HTTP and sets the provided `base_url`
    /// as the base endpoint for subsequent requests. The `base_url` can be any valid URL
    /// provided as a `String` or a type that can be converted into a `String`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - A value that can be converted into a `String`, representing the
    ///   base URL to be used for HTTP transport.
    ///
    /// # Returns
    ///
    /// Returns `Self` with the transport mechanism updated to HTTP, allowing method
    /// chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = Client::new()
    ///     .http("https://api.example.com");
    /// ```
    ///
    /// In this example, an HTTP transport is configured with `https://api.example.com`
    /// as the base URL.
    /// ```
    pub fn http(mut self, base_url: impl Into<String>) -> Self {
        self.transport = Some(Transport::Http(base_url.into()));
        self
    }

    /// Устанавливает gRPC транспорт
    pub fn grpc(mut self, addr: impl Into<String>) -> Self {
        self.transport = Some(Transport::Grpc(addr.into()));
        self
    }

    /// ```rust
    /// Sets the token value for the instance.
    ///
    /// This method takes a value that can be converted into a `String`
    /// (through the `Into<String>` trait), sets it as the token for the
    /// current instance, and returns the instance for further modification
    /// (method chaining).
    ///
    /// # Parameters
    /// - `token`: A value that can be converted into a `String`, representing
    ///   the token to be set.
    ///
    /// # Returns
    /// Returns the modified instance of the struct.
    ///
    /// # Example
    /// ```
    /// let instance = MyStruct::new().token("my_token");
    /// ```
    /// ```
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// ```rust
    /// Builds and initializes a `BlogClient` instance with the specified configuration.
    ///
    /// # Returns
    ///
    /// * `Result<BlogClient, BlogClientError>`:
    ///   - `Ok(BlogClient)` if the client is successfully built and configured.
    ///   - `Err(BlogClientError)` if there is an error during the building process.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// - If no `transport` has been specified, it will return a `BlogClientError::Config`
    ///   with the message "Transport not specified".
    /// - If there is an issue creating the `BlogClient` using the provided `transport`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = builder
    ///     .transport(my_transport)
    ///     .token(Some(my_token))
    ///     .build()
    ///     .await?;
    /// ```
    ///
    /// This example demonstrates how to use the builder pattern to create and configure a
    /// `BlogClient` instance, including specifying a transport and an optional token.
    ///
    /// pub
    pub async fn build(self) -> Result<BlogClient, BlogClientError> {
        let transport = self
            .transport
            .ok_or_else(|| BlogClientError::Config("Transport not specified".to_string()))?;

        let client = BlogClient::new(transport).await?;

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
