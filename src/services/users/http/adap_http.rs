use crate::services::users::entity::user::{User, UserResponse};
use crate::services::users::port_http::IUserHandler;
use crate::services::users::port_usecase::IUserUsecase;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

pub struct UserHandler {
    pub user_usecase: Arc<dyn IUserUsecase>,
}

impl UserHandler {
    pub fn new(user_usecase: Arc<dyn IUserUsecase>) -> Self {
        UserHandler { user_usecase }
    }
}

#[async_trait]
impl IUserHandler for UserHandler {
    async fn fetch_all_users(&self) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
        info!("HTTP handler: Fetching all users");

        match self.user_usecase.fetch_all_users().await {
            Ok(users) => {
                let response = ApiResponse {
                    success: true,
                    message: "Users fetched successfully".to_string(),
                    data: Some(users),
                };
                (StatusCode::OK, Json(response))
            }
            Err(e) => {
                error!("Failed to fetch users: {}", e);
                let response = ApiResponse::<Vec<User>> {
                    success: false,
                    message: format!("Failed to fetch users: {}", e),
                    data: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }

    async fn fetch_user_by_id(&self, id: Uuid) -> (StatusCode, Json<ApiResponse<User>>) {
        info!("HTTP handler: Fetching user by id");

        match self.user_usecase.fetch_user_by_id(id).await {
            Ok(user) => {
                let response = ApiResponse {
                    success: true,
                    message: "User fetched successfully".to_string(),
                    data: Some(user),
                };
                (StatusCode::OK, Json(response))
            }
            Err(e) => {
                error!("Failed to fetch user by id: {}", e);
                let response = ApiResponse::<User> {
                    success: false,
                    message: format!("Failed to fetch user by id: {}", e),
                    data: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }

    async fn register_user(
        &self,
        user: &mut User,
    ) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
        info!("HTTP handler: Registering user");

        match self.user_usecase.register_user(user).await {
            Ok(user) => {
                let response = ApiResponse {
                    success: true,
                    message: format!("user {} registered successfully", user.username),
                    data: Some(UserResponse {
                        id: user.id,
                        email: user.email,
                        username: user.username,
                    }),
                };
                (StatusCode::OK, Json(response))
            }
            Err(e) => {
                error!("Failed to register user: {}", e);
                let response = ApiResponse {
                    success: false,
                    message: format!("Failed to register user: {}", e),
                    data: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }
}
