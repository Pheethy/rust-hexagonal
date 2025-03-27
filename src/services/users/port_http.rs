use axum::{http::StatusCode, Json};
use async_trait::async_trait;
use mockall::automock;
use crate::services::users::http::adap_http::ApiResponse;
use crate::services::users::entity::user::User;

#[automock]
#[async_trait]
pub trait IUserHandler: Send + Sync {
    async fn fetch_all_users(&self) -> (StatusCode, Json<ApiResponse<Vec<User>>>);
}
