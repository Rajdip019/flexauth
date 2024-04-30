use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::{
    errors::Result,
    models::auth_model::{SignInPayload, SignUpPayload},
    utils::auth_utils::{sign_in, sign_up},
    AppState,
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
