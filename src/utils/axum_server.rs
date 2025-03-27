use crate::config::config_models::DotEnvConfig;
use crate::route::router::create_routes;
use crate::services::users::http::adap_http::UserHandler;
use anyhow::Result;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::OK, "What'up!").into_response()
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Rust!!").into_response()
}

pub async fn start(config: Arc<DotEnvConfig>, user_handler: Arc<UserHandler>) -> Result<()> {
    // Create main router with all routes
    let app = axum::Router::new()
        .fallback(not_found)
        .route("/health", get(health_check))
        .nest("/v1", create_routes(user_handler))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    tracing::info!("👽 Server is running on port {}", config.server.port);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shuting_down())
        .await?;
    Ok(())
}

async fn shuting_down() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("👻 Shutting down server");
}
