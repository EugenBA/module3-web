#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::blog::ProtoPost;

/// ```
/// Represents a user entity with associated properties.
///
/// This struct is used to store and manage information about a user, including
/// their unique identifier, username, email address, and the timestamp of when
/// the user was created.
///
/// # Fields
///
/// * `id` (`i64`): 
///   A unique identifier for the user, typically represented as a 64-bit integer.
///
/// * `username` (`String`): 
///   The username of the user, which is a human-readable identifier for the user.
///
/// * `email` (`String`): 
///   The email address of the user, which is used as a point of contact or for 
///   various account-related functions.
///
/// * `created_at` (`DateTime<Utc>`): 
///   A timestamp representing when the user was created,stored as a `DateTime` object 
///   in UTC format.
///
/// # Traits
///
/// This struct derives the following traits:
///
/// * `Debug`: Enables formatting and debugging of the struct.
/// * `Clone`: Allows the struct to be duplicated.
/// * `Serialize`: Enables the struct to be serialized, typically for converting into formats like JSON.
/// * `Deserialize`: Enables the struct to be deserialized from formats like JSON.
///
/// # Example
///
/// ```
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
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// ```rust
///
/// A data structure representing a blog post or article.
///
/// This struct is used to model information about a post, including its
/// metadata, content, and association with an author. It is serializable
/// and deserializable for use in various contexts, such as APIs or
/// databases.
///
/// # Fields
///
/// * `id` - A unique identifier for the post. Typically corresponds to a primary key in a database.
/// * `title` - The title of the post.
/// * `content` - The main content or body of the post.
/// * `author_id` - A unique identifier for the author of the post. This is used to associate the post with a user or creator.
/// * `created_at` - A timestamp indicating when the post was created. The timestamp is in UTC.
/// * `updated_at` - An optional timestamp indicating the last time the post was updated. If the post has not been updated, this field is `None`.
///
/// # Traits
///
/// This struct derives the following traits:
/// * `Debug` - Enables formatting with `{:?}` for debugging purposes.
/// * `Clone` - Allows the struct to be cloned, creating an identical copy.
/// * `Serialize` - Allows the struct to be serialized, e.g., to JSON or other formats.
/// * `Deserialize` - Allows the struct to be deserialized from supported data formats.
///
/// # Example
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
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(not(target_arch = "wasm32"))]
impl Post{
    fn timestamp_to_chrono(timestamp: Option<prost_types::Timestamp>) -> DateTime<Utc>{
        if let Some(ts) = timestamp {
            let system_time = std::time::UNIX_EPOCH
                + Duration::from_secs(ts.seconds as u64)
                + Duration::from_nanos(ts.nanos as u64);
            DateTime::<Utc>::from(system_time)
        }
        else {
            DateTime::<Utc>::default()
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl From<ProtoPost> for Post{
    fn from(value: ProtoPost) -> Self {
        Self{ id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: Post::timestamp_to_chrono(value.created_at),
            updated_at: Some(Post::timestamp_to_chrono(value.updated_at))
        }
    }

}

/// ```
/// Represents a response structure that encapsulates data associated with a user's request.
///
/// This struct is Serializable, Deserializable, Debuggable, and Cloneable, making it versatile for 
/// various use cases such as API responses and inter-service communication.
///
/// Fields:
/// - `posts` (Option<Vec<Post>>): Optional field representing a list of `Post` objects. This may
///   contain data for related posts or may be `None` if no posts are available.
/// - `id` (Option<i64>): Optional field representing an identifier for the response, user, or
///   associated entity. This may be `None` if the ID is not applicable or provided.
/// - `user` (Option<String>): Optional field containing the username or user identifier. Can be
///   `None` if the user information is not provided.
/// - `token` (Option<String>): Optional field for a token associated with the response, such as
///   an authentication or session token. May be `None` if the token is not required or available.
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub posts: Option<Vec<Post>>,
    pub id: Option<i64>,
    pub user: Option<String>,
    pub token: Option<String>,
}

/// ```
/// Represents a request payload for registering a new user.
///
/// This struct is used to encapsulate the necessary information
/// required during user registration, such as username, email, 
/// and password.
///
/// # Fields
///
/// * `username` - A string representing the desired username of the new user.
/// * `email` - A string containing the email address of the new user.
/// * `password` - A string holding the plain-text password for the new account, 
///                which should be securely handled.
///
/// # Traits
///
/// * `Debug` - Enables printing the struct for debugging purposes.
/// * `Clone` - Allows for creating a duplicate of the struct.
/// * `Serialize` - Allows the struct to be serialized into formats 
///                 like JSON.
/// * `Deserialize` - Allows the struct to be deserialized from formats 
///                   like JSON.
///
/// # Example
///
/// ```
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
    pub username: String,
    pub email: String,
    pub password: String,
}

/// ```rust
/// Represents a login request payload.
///
/// This struct is used to encapsulate the username and password 
/// provided by a user attempting to log in. It derives several traits 
/// to enable debugging, cloning, and serialization.
///
/// # Derived Traits
/// - `Debug`: Allows for formatting the struct using the `{:?}` formatter.
/// - `Clone`: Enables deep cloning of the struct.
/// - `Serialize` and `Deserialize`: Facilitates serialization and deserialization 
///   for use with formats like JSON.
///
/// # Fields
/// - `username` (`String`): 
///   The username provided by the user during login.
/// - `password` (`String`): 
///   The password associated with the username.
///
/// # Examples
/// ```
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
    pub username: String,
    pub password: String,
}

/// ```rust
/// Represents a request to create a new post.
///
/// # Fields
///
/// * `title` - A `String` containing the title of the post. 
///   This field is required and should not be empty.
///
/// * `content` - A `String` containing the content/body of the post.
///   This field is required and should not be empty.
///
/// # Derives
///
/// * `Debug` - Allows formatting of the structure for debugging purposes.
/// * `Clone` - Enables deep copying of the structure.
/// * `Serialize` - Allows the structure to be serialized (e.g., for converting into JSON).
/// * `Deserialize` - Allows the structure to be deserialized (e.g., for converting from JSON).
///
/// # Usage
///
/// This structure is typically used as part of an API for creating posts. 
/// Ensure you validate the fields before processing the request.
///
/// ```
/// let create_post_request = CreatePostRequest {
///     title: String::from("My First Post"),
///     content: String::from("This is the content of my first post."),
/// };
/// ```
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

/// ```rust
/// A data structure representing a request to update a blog post.
///
/// This structure is used to encapsulate the data required for updating an existing post,
/// including the post's unique identifier, title, and content. It is serializable and 
/// deserializable to facilitate communication between services, and can also be cloned and 
/// debugged for development purposes.
///
/// # Fields
///
/// * `id` - The unique identifier of the post to be updated. This value must correspond to an 
///          existing post in the system.
/// * `title` - The new title for the post. This should be a non-empty string representing the 
///             desired title.
/// * `content` - The updated content of the post. This should be a non-empty string containing
///               the body of the post.
///
/// # Traits
///
/// The `UpdatePostRequest` struct derives the following traits:
///
/// * `Debug` - Enables formatting of the struct using the `{:?}` formatter, useful during debugging.
/// * `Clone` - Allows for creating duplicate instances of the struct.
/// * `Serialize` - Enables converting the struct into a format suitable for transmission or storage, 
///                 such as JSON.
/// * `Deserialize` - Allows for creating an instance of the struct from serialized data, such as JSON.
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
}

/// ```
/// Represents the response structure for a paginated list of posts.
///
/// This struct is typically used to return data from an API endpoint that
/// retrieves a collection of posts. It includes the list of posts as well
/// as pagination metadata. It derives common traits for flexibility
/// in debugging, cloning, and serialization.
///
/// # Fields
///
/// * `posts` - A vector containing the list of posts (`Post` objects) retrieved
///   in the current response.
/// * `total` - An optional total count of posts available in the data source
///   (useful for client-side pagination). If `None`, the total count is unknown
///   or not provided.
/// * `offset` - The starting index (zero-based) of the posts returned in this
///   response. This is usually used for paginated queries to indicate the
///   position in the full list.
/// * `limit` - The maximum number of posts that can be included in the
///   response. This defines the page size or batch size for the retrieval.
///
/// # Traits
///
/// - `Debug`: Enables debug formatting for `ListPostsResponse`.
/// - `Clone`: Allows creating a deep copy of `ListPostsResponse`.
/// - `Serialize` and `Deserialize`: Enables serialization and deserialization
///   support for the struct, which is particularly useful for APIs that exchange
///   JSON or other structured data formats.
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: Option<u64>,
    pub offset: i64,
    pub limit: i64,
}

/// ```
/// Represents a request structure for listing posts with optional filtering and pagination.
///
/// This structure is used to specify query parameters when fetching a list of posts
/// from a data source. It allows for optional pagination and filtering by the author's ID.
///
/// # Fields
///
/// * `offset` - An optional parameter specifying the offset to start fetching posts from. 
///   Typically used for pagination. Accepts an `Option<i64>`.
///
/// * `limit` - An optional parameter defining the maximum number of posts to fetch.
///   Useful for limiting the result set size. Accepts an `Option<i64>`.
///
/// * `author_id` - An optional parameter to filter the posts by the author's unique identifier.
///   Accepts an `Option<String>`.
///
/// # Example
/// ```
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
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub author_id: Option<String>,
}

/// ```rust
/// A struct representing a user for storage in a WebAssembly (wasm32) environment.
///
/// This struct is only compiled and used when the target architecture 
/// is `wasm32`. It is serializable and deserializable, making it suitable 
/// for use in scenarios where data serialization is required.
///
/// # Attributes
///
/// * `id` - A 64-bit integer representing the unique identifier of the user.
/// * `username` - A `String` representing the username of the user.
///
/// # Derives
///
/// * `Debug` - Enables formatting of the struct using the `{:?}` formatter.
/// * `Clone` - Allows the struct to be cloned, producing a copy of its value.
/// * `Serialize` - Enables the struct to be serialized, typically for storage or transmission.
/// * `Deserialize` - Enables the struct to be deserialized, reconstructing it from serialized data.
///
/// # Conditional Compilation
///
/// The `#[cfg(target_arch = "wasm32")]` attribute ensures that this struct 
/// is only included in builds targeting WebAssembly (wasm32).
/// ```
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUser {
    pub id: i64,
    pub username: String,
}
