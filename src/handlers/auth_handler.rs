use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::{
    errors::Result,
    models::auth_model::{SignInPayload, SignUpPayload},
    utils::auth_utils::{signIn, signUp},
    AppState,
};

#[debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    payload: Json<SignUpPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: signup_handler called");

    let res = signUp(&state.mongo_client, payload).await.unwrap();

    Ok(res)
}

pub async fn signin_handler(
    State(state): State<AppState>,
    payload: Json<SignInPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: signin_handler called");

    let res = signIn(&state.mongo_client, payload).await?;

    Ok(res)
}
