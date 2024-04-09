use axum::{routing::get, Router};

use crate::handlers::health_check_handler::health_check_handler;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check_handler))
}