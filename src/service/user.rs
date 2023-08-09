use bcrypt::{hash, DEFAULT_COST};
use sqlx::{MySql, Pool};

use crate::{
    dto::user::{CreateUserDto, UserResponseDto},
    repository::user::UserRepository,
    utils::error::Error,
};

#[derive(Clone)]
pub struct UserService {
    pub user_repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        let user_repository = UserRepository {};
        Self { user_repository }
    }

    pub async fn get_user_by_id(
        &self,
        pool: &Pool<MySql>,
        user_id: u64,
    ) -> Result<UserResponseDto, Error> {
        let user = self.user_repository.find_by_id(pool, user_id).await?;

        let response = UserResponseDto::new(&user);
        Ok(response)
    }

    pub async fn create_user(
        &self,
        pool: &Pool<MySql>,
        request: &CreateUserDto,
    ) -> Result<(), Error> {
        let password = hash(request.get_password(), DEFAULT_COST).unwrap();

        self.user_repository
            .insert(
                pool,
                &request.get_username(),
                &password,
                &request.get_name(),
                &request.get_age(),
            )
            .await?;

        Ok(())
    }
}
