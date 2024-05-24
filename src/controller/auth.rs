use axum::{extract::State, routing::post, Router};

use crate::{
    dto::auth::{JwtResonse, LoginRequest}, utils::error::Error, GlobalState
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
    ) -> Result<JwtResonse, Error> {
        let response = auth_service
            .login(&*pool, &request)
            .await?;
        Ok(response)
    }
    Router::new().route("/login", post(login_handler))
}
