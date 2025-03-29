use crate::services::users::http::adap_http::UserHandler;
use crate::services::users::port_http::IUserHandler;
use axum::{extract::State, routing::get, Router};
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
    let id = Uuid::parse_str(&id).expect("Invalid UUID format");
    user_handler.fetch_user_by_id(id).await
}

pub fn register_user(user_handler: Arc<UserHandler>) -> Router {
    let user_handler_trait: Arc<dyn IUserHandler> = user_handler;

    Router::new()
        .route("/users", get(fetch_all_users_handler))
        .route("/user/{id}", get(fetch_user_by_id_handler))
        .with_state(user_handler_trait)
}
