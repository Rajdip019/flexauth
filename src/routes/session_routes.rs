use axum::{extract::State, routing::post, Router};

use crate::{handlers::session_handler::{delete, delete_all, get_all_from_uid, refresh_session, revoke, revoke_all, verify_session}, AppState};

pub fn routes(State(state): State<AppState>) -> Router {
    let session_routes = Router::new()
    .route("/verify", post(verify_session))
    .route("/get_all_from_uid", post(get_all_from_uid))
        .route("/refresh-session", post(refresh_session))
        .route("/revoke", post(revoke))
        .route("/revoke-all", post(revoke_all))
        .route("/delete", post(delete))
        .route("/delete-all", post(delete_all));

    Router::new().nest("/session", session_routes).with_state(state)
}