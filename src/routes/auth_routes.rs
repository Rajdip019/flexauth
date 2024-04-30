use axum::{extract::State, routing::post, Router};

use crate::{
    handlers::auth_handler::{signin_handler, signup_handler},
    AppState,
};

pub fn routes(State(state): State<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler));

    Router::new().nest("/auth", auth_routes).with_state(state)
}
