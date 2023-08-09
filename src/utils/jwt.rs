use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    GlobalState,
    utils::error::Error, config::access_token_secret
};

use super::error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: u64,
    role: String,
    exp: i64,
}

impl Claims {

    pub fn init(id: u64, role: String, exp: i64) -> Self{
        Self{ id, role,  exp}
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_role(&self) -> String {
        self.role.clone()
    }

    pub fn get_token_expire(&self) -> i64 {
        self.exp
    }

    pub fn is_token_expired(&self) -> bool {
        let currrent = Utc::now().timestamp() as i64;
        currrent > self.exp
    }

    pub fn encode_access_token(&self) -> Result<String, Error> {
        let key = EncodingKey::from_secret(access_token_secret().as_bytes());

        let token = encode(&Header::default(), &self, &key)
        .map_err(|_| {
            Error {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: "ERR".to_string(),
                error_message: "Something wrong".to_string(),
            }
        })?;
        Ok(token)
    }
}

#[async_trait]
impl FromRequestParts<GlobalState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::InvalidToken)
                .unwrap();

        let key = DecodingKey::from_secret(access_token_secret().as_bytes());
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_| {
                AuthError::InvalidToken
            })?;
        let claims = token_data.claims;

        // Check token expired
        if claims.is_token_expired() {
            return Err(AuthError::ExpiredToken);
        }

        Ok(claims)
    }
}
