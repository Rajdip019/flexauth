use axum::{routing::post, Router};

use crate::handlers::session_handler::verify_jwt_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/verify-session", post(verify_jwt_handler))
}