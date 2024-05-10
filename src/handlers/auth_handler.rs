use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::{
    core::session::Session, errors::{Error, Result}, models::{auth_model::{SignInPayload, SignUpPayload}, session_model::{RevokeSessionsPayload, RevokeSessionsResult}}, utils::auth_utils::{sign_in, sign_up}, AppState
};

#[debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    payload: Json<SignUpPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: signup_handler called");

    match sign_up(&state.mongo_client, payload).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

pub async fn signin_handler(
    State(state): State<AppState>,
    payload: Json<SignInPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: signin_handler called");

    match sign_in(&state.mongo_client, payload).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

pub async fn signout_handler(
    State(state): State<AppState>,
    payload: Json<RevokeSessionsPayload>,
) -> Result<Json<RevokeSessionsResult>> {
    println!(">> HANDLER: signout_handler called");

    if payload.session_id.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    match Session::revoke(&state.mongo_client, &payload.session_id).await {
        Ok(_) => Ok(Json(
            RevokeSessionsResult {
                message: "Session revoked successfully".to_string(),
            }
        )),
        Err(e) => Err(e),
    }

}
