use crate::services::users::entity::user::User;
use crate::services::users::http::adap_http::ApiResponse;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait IUserHandler: Send + Sync {
    async fn fetch_all_users(&self) -> (StatusCode, Json<ApiResponse<Vec<User>>>);
    async fn fetch_user_by_id(&self, id: Uuid) -> (StatusCode, Json<ApiResponse<User>>);
}
