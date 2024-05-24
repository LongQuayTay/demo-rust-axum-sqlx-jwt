use bcrypt::verify;
use chrono::{Duration, Utc};
use sqlx::{MySql, Pool};

use crate::{
    config::access_token_duration,
    dto::auth::{JwtResonse, LoginRequest},
    repository::user::UserRepository,
    utils::{error:: Error, jwt::Claims},
};

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: UserRepository,
}

impl AuthService {
    pub fn new() -> Self {
        let user_repository = UserRepository {};
        Self { user_repository }
    }

    pub async fn login(
        &self,
        pool: &Pool<MySql>,
        request: &LoginRequest,
    ) -> Result<JwtResonse, Error> {
        let user = self.user_repository.find_by_username(pool, request.get_username()).await?;
        let password = user.get_password();
        let verify =  verify(request.get_password(), &password).unwrap();
        if !verify {
            return Err(Error::UNAUTHORIZED)
        }

        let duration = Duration::hours(access_token_duration());
        let expire = Utc::now().checked_add_signed(duration).unwrap().timestamp();

        let claims = Claims::init(user.get_id(), "Admin".to_string(), expire);
        let access_token = claims.encode_access_token()?;
        let token_type = "Bearer".to_string();

        let response = JwtResonse::init(access_token, token_type);
        Ok(response)
    }
}
