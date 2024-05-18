use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn with_api_key(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    println!("Checking for API key...");

    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|key| key.to_str().ok());

    let expected_api_key = env::var("X_API_KEY").expect("X_API_KEY must be set");


    if let Some(key) = api_key {
        if key == expected_api_key {
            return Ok(next.run(req).await);
        }
    }
    // error with message of x-api-key not match
    
    Ok(Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Invalid API key"))
        .unwrap())
    

}
