use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorResponseDetails,
}

#[derive(Serialize)]
struct ErrorResponseDetails {
    status: u16,
    message: String,
    req_uuid: String,
}

async fn log_request(uuid: &str, req_method: &str, uri: &str, status: u16, message: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    println!(
        "RequestLog: {{ uuid: {}, timestamp: {}, req_method: {}, uri: {}, status: {}, message: {} }}",
        uuid, timestamp, req_method, uri, status, message
    );
}

pub async fn with_api_key(req: Request<Body>, next: Next) -> Result<Response<Body>, StatusCode> {
    let expected_api_key = env::var("X_API_KEY").unwrap_or_default();
    let api_key = req.headers().get("x-api-key").and_then(|key| key.to_str().ok());

    let uuid = Uuid::new_v4().to_string();
    let req_method = req.method().to_string();
    let req_uri = req.uri().to_string();

    if expected_api_key.is_empty() {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let message = "Internal Server Error: API key is not configured".to_string();

        log_request(&uuid, &req_method, &req_uri, status.as_u16(), &message).await;

        let error_response = ErrorResponse {
            error: ErrorResponseDetails {
                status: status.as_u16(),
                message,
                req_uuid: uuid.clone(),
            },
        };

        return Ok((status, Json(error_response)).into_response());
    }

    if let Some(key) = api_key {
        if key == expected_api_key {
            return Ok(next.run(req).await);
        }
    }

    let status = StatusCode::UNAUTHORIZED;
    let message = "Unauthorized: Invalid API key".to_string();

    log_request(&uuid, &req_method, &req_uri, status.as_u16(), &message).await;

    let error_response = ErrorResponse {
        error: ErrorResponseDetails {
            status: status.as_u16(),
            message,
            req_uuid: uuid.clone(),
        },
    };

    Ok((status, Json(error_response)).into_response())
}
