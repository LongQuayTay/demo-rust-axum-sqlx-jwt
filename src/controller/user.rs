use axum::{extract::State, http::StatusCode, routing::{get, post}, Router};

use crate::{
    dto::user::{CreateUserDto, UserResponseDto},
    utils::{error::Error, jwt::Claims},
    GlobalState,
};

pub fn user_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/user",
        Router::new()
            // Post /user
            .merge(create_user())
            // Get /user/get-user-info
            .merge(get_user_info()),
    )
}

pub fn get_user_info() -> Router<GlobalState> {
    async fn get_user_info_handler(
        State(GlobalState { pool, user_service, ..}): State<GlobalState>,
        claims: Claims
    ) -> Result<UserResponseDto, Error> {
        Ok(user_service.get_user_by_id(&*pool, claims.get_id()).await?)
    }
    Router::new().route("/get-user-info", get(get_user_info_handler))
}

pub fn create_user() -> Router<GlobalState> {
    async fn create_user_handler(
        State(GlobalState { pool, user_service, ..}): State<GlobalState>,
        request: CreateUserDto,
    ) -> Result<(StatusCode, String), Error> {
        user_service.create_user(&pool, &request).await?;
        Ok((StatusCode::CREATED, "Success".to_string()))
    }
    Router::new().route("/", post(create_user_handler))
}
