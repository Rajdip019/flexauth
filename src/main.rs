use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::{middleware, Router};
use dotenv::dotenv;
use middlewares::res_log::main_response_mapper;
use middlewares::with_api_key::with_api_key;
use mongodb::Client;
use std::error::Error;

mod config;
mod core;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod traits;
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
    // Define routes where middleware is applied
    let protected_routes = Router::new()
        .merge(routes::auth_routes::routes(State(app_state.clone())))
        .merge(routes::user_routes::routes(State(app_state.clone())))
        .merge(routes::password_routes::routes(State(app_state.clone())))
        .merge(routes::session_routes::routes(State(app_state.clone())))
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(with_api_key));

    // Define routes where middleware is not applied
    let public_routes = Router::new().route("/", get(root_handler));

    // Combine public and protected routes
    let app = Router::new()
        .nest("/api", protected_routes)
        .nest("/", public_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root_handler() -> Html<&'static str> {
    let html_content = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>FlexAuth</title>
            </head>
            <body>
                <h1>Welcome to FlexAuth</h1>
                <p>Your own flexible, blazingly fast 🦀, and secure in-house authentication system.</p>
                <p>Here's the <a href="https://documenter.getpostman.com/view/18827552/2sA3JT4Jmd">API documentation</a>.</p>
            </body>
        </html>
    "#;
    Html(html_content)
}
