use axum::Json;
use axum_macros::debug_handler;
use serde_json::{json, Value};

use crate::{errors::{Error, Result}, models::session_model::VerifyJwt, utils::session_utils::verify_jwt};

#[debug_handler]
pub async fn verify_jwt_handler(
    payload: Json<VerifyJwt>,
) -> Result<Json<Value>> {
    // check if the token is not empty
    if payload.token.is_empty() {
        return Err(Error::InvalidPayload { message: "Invalid payload passed".to_string() });
    }

    // verify the token
    let _ = match verify_jwt(&payload.token) {
        Ok(val) => return Ok(Json(json!(*val))),
        Err(e) => return Err(Error::InvalidToken { message: e.to_string() }),
    };
}
