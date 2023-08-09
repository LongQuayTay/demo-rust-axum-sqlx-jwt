use axum::{extract::State, routing::post, Router};

use crate::{
    dto::auth::{JwtResonse, LoginRequest},
    utils::error::AuthError,
    GlobalState,
};

pub fn auth_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/auth",
        Router::new()
            // POST /auth/login
            .merge(login()),
    )
}

pub fn login() -> Router<GlobalState> {
    async fn login_handler(
        State(GlobalState {
            pool, auth_service, ..
        }): State<GlobalState>,
        request: LoginRequest,
    ) -> Result<JwtResonse, AuthError> {
        let response = auth_service
            .login(&*pool, &request)
            .await
            .map_err(|_| AuthError::TokenCreation)?;
        Ok(response)
    }
    Router::new().route("/login", post(login_handler))
}
