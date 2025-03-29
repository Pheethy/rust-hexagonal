use crate::services::users::entity::user;
use crate::services::users::http::adap_http::UserHandler;
use crate::services::users::port_http::IUserHandler;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::{extract::Multipart, extract::State, routing::get, routing::post, Router};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

async fn fetch_all_users_handler(
    State(user_handler): State<Arc<dyn IUserHandler>>,
) -> impl axum::response::IntoResponse {
    user_handler.fetch_all_users().await
}

async fn fetch_user_by_id_handler(
    State(user_handler): State<Arc<dyn IUserHandler>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    match Uuid::parse_str(&id) {
        Ok(id) => user_handler.fetch_user_by_id(id).await.into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid UUID format" })),
        )
            .into_response(),
    }
}

async fn register_user_handler(
    State(user_handler): State<Arc<dyn IUserHandler>>,
    multipart: Multipart,
) -> impl IntoResponse {
    let mut user = user::new_user_with_multipart(multipart).await;
    user_handler.register_user(&mut user).await
}

pub fn register_user(user_handler: Arc<UserHandler>) -> Router {
    let user_handler_trait: Arc<dyn IUserHandler> = user_handler;

    Router::new()
        .route("/users", get(fetch_all_users_handler))
        .route("/user/{id}", get(fetch_user_by_id_handler))
        .route("/user", post(register_user_handler))
        .with_state(user_handler_trait)
}
