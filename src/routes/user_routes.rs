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
    let user_routes = Router::new()
        .route("/get-all", get(get_all_users_handler))
        .route("/get-from-email", post(get_user_email_handler))
        .route("/get-from-id", post(get_user_id_handler))
        .route("/update", post(update_user_handler))
        .route("/toggle-account-active-status", post(toggle_user_activation_status))
        .route("/update-role", post(update_user_role_handler))
        .route("/delete", post(delete_user_handler));

    Router::new().nest("/user", user_routes).with_state(state)
}
