use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message, code) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials", "ERR")
            }
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error",
                "ERR",
            ),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token", "ERR"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token", "ERR"),
        };
        let body = Json(json!({
            "code" : code,
            "error": error_message
        }));
        (status, body).into_response()
    }
}

#[derive(Debug)]
pub struct Error {
    pub status_code: StatusCode,
    pub error_message: String,
    pub error_code: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error_code" : self.error_code,
            "error_message": self.error_message
        }));
        (self.status_code, body).into_response()
    }
}
