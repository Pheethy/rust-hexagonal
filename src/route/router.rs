use crate::services::users::http::adap_http::UserHandler;
use crate::services::users::port_http::IUserHandler;
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

// Handler function for fetch_all_users
async fn fetch_all_users_handler(
    State(user_handler): State<Arc<dyn IUserHandler>>,
) -> impl axum::response::IntoResponse {
    user_handler.fetch_all_users().await
}

pub fn create_routes(user_handler: Arc<UserHandler>) -> Router {
    // Cast UserHandler to Arc<dyn IUserHandler>
    let user_handler_trait: Arc<dyn IUserHandler> = user_handler;

    Router::new()
        .route("/users", get(fetch_all_users_handler))
        .with_state(user_handler_trait)
}
