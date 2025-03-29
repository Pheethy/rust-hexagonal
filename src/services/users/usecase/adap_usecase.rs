use crate::services::users::entity::user::User;
use crate::services::users::port_repository::IUserRepository;
use crate::services::users::port_usecase::IUserUsecase;
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub struct UserUsecase {
    pub user_repository: Box<dyn IUserRepository>,
}

impl UserUsecase {
    pub fn new(user_repository: Box<dyn IUserRepository>) -> Self {
        UserUsecase { user_repository }
    }
}

#[async_trait]
impl IUserUsecase for UserUsecase {
    async fn fetch_all_users(&self) -> Result<Vec<User>> {
        match self.user_repository.fetch_all_users().await {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::error!("Failed to fetch users: {}", e);
                Err(anyhow::anyhow!("Failed to fetch users: {}", e))
            }
        }
    }

    async fn fetch_user_by_id(&self, id: Uuid) -> Result<User> {
        match self.user_repository.fetch_user_by_id(id).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::error!("Failed to fetch user by id: {}", e);
                Err(anyhow::anyhow!("Failed to fetch user by id: {}", e))
            }
        }
    }

    async fn register_user(&self, user: User) -> Result<User> {
        match self.user_repository.register_user(user).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::error!("Failed to register user: {}", e);
                Err(anyhow::anyhow!("Failed to register user: {}", e))
            }
        }
    }
}
