use axum::{
    extract::rejection::JsonRejection, http::StatusCode, response::{IntoResponse, Response}, Json
};
use serde_json::json;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error{

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Json Web Token errors")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error("Json parse error")]
    Json(#[from] JsonRejection),

    #[error("UNAUTHORIZED")]
    UNAUTHORIZED
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error_code" : "",
            "error_message": "ERR"
        }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}