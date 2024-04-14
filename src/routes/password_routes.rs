use axum::{extract::State, routing::post, Router};

use crate::{handlers::password_handler::{forget_reset_password_handler, forgot_password_request_handler, reset_password_handler}, AppState};

pub fn routes(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/reset-password", post(reset_password_handler))
        .route("/forget-password-request", post(forgot_password_request_handler))
        .route("/forget-password-reset/:id", post(forget_reset_password_handler))
        .with_state(state)
}