use axum::{extract::State, middleware, routing::get, Router};
use middlewares::res_log::main_response_mapper;
use mongodb::Client;
use utils::hashing_utils::{create_dek, encrypt_data, decrypt_data};
use std::error::Error;
use tokio;

use utils::hashing_utils::salt_and_hash_password;

mod handlers;
mod routes;
mod models;
mod errors;
mod config;
mod middlewares;
mod utils;

#[derive(Clone)]
struct AppState {
    mongo_client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {   
    let password = "test_password";
    let hashed_and_salted_pass = salt_and_hash_password(password);
    println!("Hashed password: {:?}", hashed_and_salted_pass.password);
    println!("Salt: {:?}", hashed_and_salted_pass.salt);
    let key = create_dek();
    println!("Key: {:?}", key);
    let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &key);
    let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &key);
    println!("Encrypted password: {:?}", encrypted_password);
    println!("Encrypted salt: {:?}", encrypted_salt);
    let decrypted_password = decrypt_data(&encrypted_password, &key);
    let decrypted_salt = decrypt_data(&encrypted_salt, &key);
    println!("Decrypted password: {:?}", decrypted_password);
    println!("Decrypted salt: {:?}", decrypted_salt);
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
    axum::serve::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}

async fn root_handler() -> &'static str {
    "Hello, welcome to in-house auth service!"
}

