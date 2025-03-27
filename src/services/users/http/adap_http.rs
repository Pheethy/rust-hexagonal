use crate::services::users::entity::user::User;
use crate::services::users::port_http::IUserHandler;
use crate::services::users::port_usecase::IUserUsecase;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;
use tracing::{error, info};

// Define ApiResponse for responses
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

// UserHandler struct
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
}
