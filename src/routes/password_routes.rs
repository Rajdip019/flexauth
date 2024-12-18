use axum::{extract::State, routing::post, Router};

use crate::{handlers::password_handler::{forget_password_request_handler, forget_password_reset_handler, reset_password_handler}, AppState};

pub fn routes(State(state): State<AppState>) -> Router {
    let password_rotes = Router::new()
        .route("/reset", post(reset_password_handler))
        .route("/forget-request", post(forget_password_request_handler))
        .route("/forget-reset/:id", post(forget_password_reset_handler));


    Router::new().nest("/password", password_rotes).with_state(state)
}