use axum::http::{header, Method};
use tower_http::cors::CorsLayer;

pub fn port() -> u16 {
    dotenv::var("PORT")
        .expect("PORT is not set !!!")
        .parse()
        .expect("PORT is not a number")
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_headers([
            header::ORIGIN,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::ACCESS_CONTROL_ALLOW_METHODS,
            header::ACCESS_CONTROL_ALLOW_HEADERS,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
}

pub fn database_host() -> String {
    dotenv::var("DATABASE_HOST")
        .expect("DATABASE HOST is not set !!!")
}

pub fn database_port() -> u16 {
    dotenv::var("DATABASE_PORT")
        .expect("DATABASE PORT is not set !!!")
        .parse()
        .expect("DATABASE PORT is not a number")
}

pub fn database_username() -> String {
    dotenv::var("DATABASE_USERNAME")
        .expect("DATABASE USERNAME is not set !!!")
}

pub fn database_password() -> String {
    dotenv::var("DATABASE_PASSWORD")
        .expect("DATABASE PASSWORD is not set !!!")
}

pub fn database_dbname() -> String {
    dotenv::var("DATABASE_DBNAME")
        .expect("DATABASE DBNAME is not set !!!")
}

pub fn access_token_secret() -> String {
    dotenv::var("ACCESS_TOKEN_SECRET")
        .expect("ACCESS TOKEN SECRET is not set !!!")
}

pub fn access_token_duration() -> i64 {
    dotenv::var("ACCESS_TOKEN_DURATION")
        .expect("ACCESS TOKEN DURATION is not set !!!")
        .parse()
        .expect("ACCESS TOKEN DURATION PORT is not a number")
}