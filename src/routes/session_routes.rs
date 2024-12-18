use axum::{extract::State, routing::{get, post}, Router};

use crate::{
    handlers::session_handler::{
        delete_all_handler, delete_handler, get_all_from_uid_handler, get_all_handler, get_details_handler, refresh_session_handler, revoke_all_handler, revoke_handler, verify_session_handler
    }, AppState
};

pub fn routes(State(state): State<AppState>) -> Router {
    let session_routes = Router::new()
        .route("/verify", post(verify_session_handler))
        .route("/get-all", get(get_all_handler))
        .route("/get-all-from-uid", post(get_all_from_uid_handler))
        .route("/get-details", post(get_details_handler))
        .route("/refresh-session", post(refresh_session_handler))
        .route("/revoke", post(revoke_handler))
        .route("/revoke-all", post(revoke_all_handler))
        .route("/delete", post(delete_handler))
        .route("/delete-all", post(delete_all_handler));

    Router::new()
        .nest("/session", session_routes)
        .with_state(state)
}
