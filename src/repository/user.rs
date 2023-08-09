use axum::http::StatusCode;
use sqlx::{MySql, Pool};

use crate::{entity::user::User, utils::error::Error};

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(&self, pool: &Pool<MySql>, user_id: u64) -> Result<User, Error> {
        let sql = "select * from users where id = ?";

        let row = sqlx::query_as::<_, User>(sql)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Server running on Port: {} ", e);
                Error {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: "ERR".to_string(),
                    error_message: "Something wrong".to_string(),
                }
            })?;
        Ok(row)
    }

    pub async fn insert(
        &self,
        pool: &Pool<MySql>,
        username: &String,
        password: &String,
        name: &String,
        age: &Option<u8>,
    ) -> Result<u64, Error> {
        let sql = "INSERT INTO users (username, name, password, age) VALUES (?,?,?,?)";

        let id = sqlx::query(&sql)
            .bind(username)
            .bind(name)
            .bind(password)
            .bind(age)
            .execute(pool)
            .await
            .map_err(|_| Error {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: "ERR".to_string(),
                error_message: "Something wrong".to_string(),
            })
            .unwrap()
            .last_insert_id();
        Ok(id)
    }

    pub async fn find_by_username(
        &self,
        pool: &Pool<MySql>,
        username: String,
    ) -> Result<User, Error> {
        let sql = "select * from users where username = ?";

        let row = sqlx::query_as::<_, User>(sql)
            .bind(username)
            .fetch_one(pool)
            .await
            .map_err(|_| Error {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: "ERR".to_string(),
                error_message: "Something wrong".to_string(),
            })?;
        Ok(row)
    }
}
