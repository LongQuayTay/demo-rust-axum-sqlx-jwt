use axum::{response::{IntoResponse, Response}, Json, http::{StatusCode, Request}, async_trait, extract::FromRequest, body::Body};
use serde::{Deserialize, Serialize};

use crate::{utils::error::Error, GlobalState, entity::user::User};

#[derive(Serialize)]
pub struct UserResponseDto {
    id: u64,
    name: String,
    age: Option<u8>,
}

impl UserResponseDto {
    pub fn new(user: &User) -> Self {
        Self {
            id: user.get_id(),
            name: user.get_name(),
            age: user.get_age(),
        }
    }
}

impl IntoResponse for UserResponseDto{
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    username: String,
    password: String,
    name: String,
    age: Option<u8>,
}

impl CreateUserDto{
    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn get_age(&self) -> Option<u8> {
        self.age
    }
}

#[async_trait]
impl FromRequest<GlobalState, Body> for CreateUserDto {
    
    type Rejection = Error;

    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {

        let Json(body) = Json::<CreateUserDto>::from_request(req, state)
            .await
            .map_err(|_| Error {
                status_code: StatusCode::BAD_REQUEST,
                error_code: "ERR004".to_string(),
                error_message: "Wrong request body".to_string(),
            })?;

        Ok(body)
    }
}

