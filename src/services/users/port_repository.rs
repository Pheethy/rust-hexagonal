use crate::services::users::entity::user::User;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn fetch_all_users(&self) -> Result<Vec<User>>;
    async fn register_user(&self, user: User) -> Result<User>;
}
