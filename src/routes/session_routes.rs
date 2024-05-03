use axum::{extract::State, routing::post, Router};

use crate::{handlers::session_handler::{verify_session, get_all_from_uid}, AppState};

pub fn routes(State(state): State<AppState>) -> Router {
    let session_routes = Router::new()
        .route("/verify", post(verify_session))
        .route("/get_all_from_uid", post(get_all_from_uid));

    Router::new().nest("/session", session_routes).with_state(state)
}