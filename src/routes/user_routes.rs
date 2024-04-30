use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::user_handler::{
        delete_user_handler, get_all_users_handler, get_user_email_handler, get_user_id_handler,
        toggle_user_activation_status, update_user_handler, update_user_role_handler,
    },
    AppState,
};

pub fn routes(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/get-all-users", get(get_all_users_handler))
        .route("/get-user-from-email", post(get_user_email_handler))
        .route("/get-user-from-id", post(get_user_id_handler))
        .route("/update-user", post(update_user_handler))
        .route("/toggle-user-active-status", post(toggle_user_activation_status))
        .route("/update-user-role", post(update_user_role_handler))
        .route("/delete-user", post(delete_user_handler))
        .with_state(state)
}
