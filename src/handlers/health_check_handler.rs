use axum::Json;
use serde_json::{json, Value};
use crate::errors::Result;

pub async fn health_check_handler() ->  Result<Json<Value>> {
    Ok(Json(json!({"status": "ok"})))
}