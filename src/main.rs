use rust_hexagonal::config::config_loader;
use rust_hexagonal::route::router::register_user;
use rust_hexagonal::services::users::http::adap_http::UserHandler;
use rust_hexagonal::services::users::repository::adap_repository::UserRepository;
use rust_hexagonal::services::users::usecase::adap_usecase::UserUsecase;
use rust_hexagonal::utils::axum_server::start;
use rust_hexagonal::utils::postgres;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let dot_env = match config_loader::load_config() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let psql_db = match postgres::database_connect(&dot_env.database.url).await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let psql_db_arc = Arc::new(psql_db);
    /* Init Repository */
    let user_repository = UserRepository::new(Arc::clone(&psql_db_arc));

    /* Init Usecase */
    let user_usecase = UserUsecase::new(Box::new(user_repository));

    /* Init Handler */
    let user_handler = Arc::new(UserHandler::new(Arc::new(user_usecase)));

    /* Start Server */
    match start(Arc::new(dot_env), register_user(user_handler)).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}
