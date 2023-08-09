pub mod config;
pub mod controller;
pub mod service;
pub mod repository;
pub mod entity;
pub mod dto;
pub mod routes;
pub mod utils;

use axum::{
    Router,
    Server,
};
use service::{user::UserService, auth::AuthService};
use std::{net::SocketAddr, sync::Arc};

use config::*;
use dotenv::dotenv;
use routes::routes;
use sqlx::{mysql::MySqlConnectOptions, MySql, MySqlPool, Pool};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct GlobalState {
    pub pool: Arc<Pool<MySql>>,
    pub user_service: UserService,
    pub auth_service: AuthService
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool_options = MySqlConnectOptions::new()
        .host(&database_host())
        .port(database_port())
        .database(&database_dbname())
        .username(&database_username())
        .password(&database_password());

    let pool: Pool<MySql> = MySqlPool::connect_with(pool_options)
        .await
        .map_err(|e| tracing::error!("Error connecting to database: {}", e))
        .unwrap();

    let state = GlobalState {
        pool: Arc::new(pool.clone()),
        user_service: UserService::new(),
        auth_service: AuthService::new()
    };

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port()));

    let app: Router = routes(state).layer(cors()).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
    );

    tracing::info!("Server running on Port: {} ", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
