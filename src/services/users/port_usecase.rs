use crate::services::users::entity::user::User;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait IUserUsecase: Send + Sync {
    async fn fetch_all_users(&self) -> Result<Vec<User>>;
    async fn fetch_user_by_id(&self, id: Uuid) -> Result<User>;
    async fn register_user(&self, user: User) -> Result<User>;
}
