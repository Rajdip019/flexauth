use axum::{extract::State, middleware, response::IntoResponse, routing::get, Json, Router};
use mongodb::Client;
use serde_json::json;
use std::error::Error;
use crate::error::Error as ServiceError;
use tokio;
use axum::http::{Method, Uri};
use bson::Uuid;
use axum::response::Response;


mod handlers;
mod routes;
mod models;
mod error;
mod config;
mod log;

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
        .merge(routes::health_check_routes::routes())
        .merge(routes::user_routes::routes(State(app_state)))
        .layer(middleware::map_response(main_response_mapper));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}

async fn root_handler() -> &'static str {
    "Hello, welcome to in-house auth service!"
}

async fn main_response_mapper(
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    let uuid = Uuid::new();

    // Get the eventual response error
    let service_error = res.extensions().get::<ServiceError>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let client_error_body = json!({"error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
            }
        });

        println!(">> Client Error: {:?}", client_error_body);
        (*status, Json(client_error_body)).into_response()
    });

    println!(">> Server Log line - {uuid} - Error: {error:?}", uuid = uuid, error = client_status_error);

    // Build and log the request log line
    let client_error  = client_status_error.unzip().1;
    log::log_request(
        uuid.to_string(),
        req_method,
        uri,
        service_error.cloned(),
        client_error,
    ).await.unwrap(); 

    println!();
    error_response.unwrap_or(res)
}
