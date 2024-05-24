use axum::{
    async_trait,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    Json, response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{utils::error::Error, GlobalState};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl LoginRequest{

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }
}

#[derive(Serialize)]
pub struct JwtResonse {
    access_token: String,
    token_type: String
}

#[async_trait]
impl FromRequest<GlobalState, Body> for LoginRequest {
    
    type Rejection = Error;

    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {

        let Json(body) = Json::<LoginRequest>::from_request(req, state)
            .await?;
        Ok(body)
    }
}

impl JwtResonse {
    pub fn init(access_token: String, token_type: String) -> Self{
        Self{access_token, token_type}
    }
}

impl IntoResponse for JwtResonse{
    fn into_response(self) -> Response {
        let body = Json(json!({         
            "access_token" : self.access_token,
            "token_type": self.token_type
        }));
        (StatusCode::OK, body).into_response()
    }
}