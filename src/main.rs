use std::sync::Arc;

use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

use crate::{
    config::{config::Config, mail_config::MailConfig},
    database::DBClient,
    routes::create_router,
};

mod config;
mod constants;
mod database;
mod dtos;
mod error;
mod handlers;
mod mail;
mod middleware;
mod models;
mod routes;
mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
    pub mail_config: MailConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();
    let mail_config = MailConfig::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            std::process::exit(1)
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let db_client = DBClient::new(pool);

    let app_state = AppState {
        env: config.clone(),
        db_client: db_client,
        mail_config: mail_config,
    };

    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());

    println!(
        "{}",
        format!("Server is running on http://localhost:{}", &config.port)
    );
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    let _ = axum::serve(listener, app).await.unwrap();
}
