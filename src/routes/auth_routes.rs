use axum::{extract::State, routing::post, Router};

use crate::{
    handlers::auth_handler::{signin_handler, signup_handler},
    AppState,
};

pub fn routes(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .with_state(state)
}
