use super::config_models::{Database, DotEnvConfig, Server};
use super::config_stage::Stage;
use anyhow::Result;

pub fn load_config() -> Result<DotEnvConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is not set on .env file")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is not set on .env file")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is not set on .env file")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set on .env file"),
    };

    Ok(DotEnvConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_form(stage_str.as_str()).unwrap()
}
