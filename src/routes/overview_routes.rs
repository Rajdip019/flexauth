use axum::{extract::State, routing::get, Router};

use crate::{handlers::overview_handler::get_all_overview_handler, AppState};

pub fn routes(State(state): State<AppState>) -> Router {
    let overview_routes = Router::new().route("/get-all", get(get_all_overview_handler));

    Router::new()
        .nest("/overview", overview_routes)
        .with_state(state)
}
