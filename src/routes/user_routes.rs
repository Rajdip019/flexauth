use axum::{extract::State, routing::{post, get}, Router};

use crate::{handlers::user_handler::{add_user_handler, delete_user_handler, get_all_users_handler, get_user_handler, update_user_handler}, AppState};

pub fn routes(State(state) : State<AppState>) -> Router {
    Router::new()
        .route("/add-user", post(add_user_handler))
        .route("/get-all-users", get(get_all_users_handler))
        .route("/get-user", post(get_user_handler))
        .route("/update-user", post(update_user_handler))
        .route("/delete-user", post(delete_user_handler))
        .with_state(state)
}