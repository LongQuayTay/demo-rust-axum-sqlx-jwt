use axum::{
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router
};
use serde::{Deserialize, Serialize};

use crate::{GlobalState, controller::{auth::auth_routes, user::user_routes}};

pub fn routes(state: GlobalState) -> Router {
    Router::new()
        .route("/", get(say_hello))
        .route("/path/:id", get(get_path))
        .route("/path2/:path1/:path2", get(double_path))
        .route("/not-found", get(not_found))
        .route("/response", get(response))
        .route("/get-header", get(get_headers))
        .route("/json", post(send_json_body))
        .route("/query", get(send_query))          
        .merge(auth_routes())
        .merge(user_routes())
        .with_state(state)
}

async fn say_hello() -> String {
    "Hello, World".to_owned()
}

async fn get_path(Path(path): Path<String>) -> String {
    format!("Path : {} ", path)
}

async fn double_path(Path((path1, path2)): Path<(String, String)>) -> String {
    format!("Path1 : {}, Path2  {} ", path1, path2)
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn response() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(BodyJson {
            message: Some("NOT FOUND".to_owned()),
            id: "404".to_owned(),
        }),
    ).into_response()
}

async fn get_headers(header: HeaderMap) -> String {
    header
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap()
}

async fn send_json_body(Json(body): Json<BodyJson>) -> Json<BodyJson> {
    Json(body)
}

async fn send_query(Query(query): Query<BodyJson>) -> Json<BodyJson> {
    Json(query)
}

#[derive(Deserialize, Serialize)]
struct BodyJson {
    message: Option<String>,
    id: String,
}

