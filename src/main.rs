use axum::{extract::State, routing::get, Router};
use mongodb::Client;
use std::error::Error;
use tokio;

mod handlers;
mod routes;
mod models;
mod error;
mod config;

#[derive(Clone)]
struct AppState {
    mongo_client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {    
    let mongo_client = config::db_connection_handler::mongo_connection_handler().await?;
    
    // init users if not exists
    config::user_init::init_users(mongo_client.clone()).await;


    let app_state = AppState { mongo_client };
    // run the server 
    let app = Router::new()
        .route("/", get(root_handler))
        .merge(routes::user_routes::routes(State(app_state)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}

async fn root_handler() -> &'static str {
    "Hello, welcome to in-house auth service!"
}