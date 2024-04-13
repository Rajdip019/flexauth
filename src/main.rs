use axum::{extract::State, middleware, routing::get, Router};
use dotenv::dotenv;
use middlewares::res_log::main_response_mapper;
use mongodb::Client;
use std::error::Error;

mod config;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

#[derive(Clone)]
struct AppState {
    mongo_client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let mongo_client = config::db_connection_handler::connect().await?;
    // init users if not exists
    config::init::init_users(mongo_client.clone()).await;

    let app_state = AppState { mongo_client };
    // run the server
    let app = Router::new()
        .route("/", get(root_handler))
        .merge(routes::health_check_routes::routes())
        .merge(routes::user_routes::routes(State(app_state)))
        .layer(middleware::map_response(main_response_mapper));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root_handler() -> &'static str {
    "Hello, welcome to in-house auth service!"
}
