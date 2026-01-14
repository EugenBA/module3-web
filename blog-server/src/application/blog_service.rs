use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::data::blog_repository::BlogRepository;
use crate::domain::error::{DomainError, BlogError};
use crate::domain::post::{Post, CreatePost, UpdatePost};

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
        Ok(self.repo.create(author_id, create_post.clone()).await.map_err(BlogError::from)?)
    }

    #[instrument(skip(self))]
    pub async fn update_post(&self, post_id: i64, update_post: UpdatePost) -> Result<Vec<Account>, BankError> {
        self.repo.list_for_user(owner).await.map_err(BankError::from)
    }


    #[instrument(skip(self))]
    pub async fn get_post(&self, id: u32) -> Result<Account, BankError> {
        match self.repo.get(id).await.map_err(BankError::from)? {
            Some(account) => Ok(account),
            None => Err(BankError::NotFound(format!("account {}", id))),
        }
    }



    #[instrument(skip(self))]
    pub async fn delete_post(&self, id: u32, amount: i64) -> Result<Account, BankError> {
        let mut account = self.get_account(id).await?;
        let amount = Amount::new(amount).map_err(BankError::from)?;
        account.deposit(amount);
        self.repo.upsert(account.clone()).await.map_err(BankError::from)?;
        Ok(account)
    }

}


