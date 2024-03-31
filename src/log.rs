use std::time::SystemTime;

use crate::error::{ClientError, Error};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use axum::response::Result;

pub async fn log_request(
    uuid: String,
    req_method: Method,
    uri: Uri,
    service_error: Option<Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let error_type = service_error.clone().map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
    .ok()
    .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid,
        timestamp: timestamp.to_string(),

        req_path: uri.path().to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),
        
        error_type,
        error_data,
    };

    println!(">> Log Line: \n{:?}", json!(log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    // -- http request attributes
    req_path: String,
    req_method: String,

    // -- Errors attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
