use std::sync::Arc;

use tracing::instrument;

use crate::data::blog_repository::BlogRepository;
use crate::domain::error::BlogError;
use crate::domain::post::{CreatePost, Post, UpdatePost};

#[derive(Clone)]
pub struct BlogService<R: BlogRepository + 'static> {
    repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: BlogRepository + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self))]
    pub async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, BlogError> {
        let create_post = CreatePost::new(title, content);
        Ok(self
            .repo
            .create(author_id, create_post.clone())
            .await
            .map_err(BlogError::from)?)
    }

    #[instrument(skip(self))]
    pub async fn update_post(
        &self,
        post_id: i64,
        author_id: i64,
        update_post: UpdatePost,
    ) -> Result<Post, BlogError> {
        Ok(self
            .repo
            .update_post(post_id, author_id, update_post.clone())
            .await
            .map_err(BlogError::from)?)
    }

    #[instrument(skip(self))]
    pub async fn delete_post(&self, post_id: i64, author_id: i64) -> Result<(), BlogError> {
        Ok(self
            .repo
            .delete_post(post_id, author_id)
            .await
            .map_err(BlogError::from)?)
    }

    #[instrument(skip(self))]
    pub async fn get_posts(&self, author_id: i64) -> Result<Vec<Post>, BlogError> {
        Ok(self
            .repo
            .get_posts(author_id)
            .await
            .map_err(BlogError::from)?)
    }

    #[instrument(skip(self))]
    pub async fn get_post(&self, post_id: i64) -> Result<Option<Post>, BlogError> {
        Ok(self.repo.get_post(post_id).await.map_err(BlogError::from)?)
    }
}
