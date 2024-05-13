use axum::{extract::State, Json};
use axum_macros::debug_handler;

use crate::{
    core::{auth::Auth, session::Session},
    errors::{Error, Result},
    models::{
        auth_model::{SignInOrSignUpResponse, SignInPayload, SignUpPayload},
        session_model::{RevokeSessionsPayload, RevokeSessionsResult},
    },
    utils::validation_utils::Validation,
    AppState,
};

#[debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    payload: Json<SignUpPayload>,
) -> Result<Json<SignInOrSignUpResponse>> {
    println!(">> HANDLER: signup_handler called");

    // check if the payload is empty
    if payload.name.is_empty()
        || payload.email.is_empty()
        || payload.role.is_empty()
        || payload.password.is_empty()
    {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    if !Validation::password(&payload.password) {
        return Err(Error::InvalidPayload {
            message: "The password must contain at least one alphabetic character (uppercase or lowercase), at least one digit, and must be at least 8 characters long.".to_string(),
        });
    }

    match Auth::sign_up(
        &state.mongo_client,
        &payload.name,
        &payload.email,
        &payload.role,
        &payload.password,
    )
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn signin_handler(
    State(state): State<AppState>,
    payload: Json<SignInPayload>,
) -> Result<Json<SignInOrSignUpResponse>> {
    println!(">> HANDLER: signin_handler called");
    // check if the payload is empty
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    if !Validation::email(&payload.email) {
        return Err(Error::InvalidPayload {
            message: "Invalid Email".to_string(),
        });
    }

    if !Validation::password(&payload.password) {
        return Err(Error::InvalidPayload {
            message: "The password must contain at least one alphabetic character (uppercase or lowercase), at least one digit, and must be at least 8 characters long.".to_string(),
        });
    }

    match Auth::sign_in(&state.mongo_client, &payload.email, &payload.password).await {
        Ok(res) => Ok(Json(res)),
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
        Ok(_) => Ok(Json(RevokeSessionsResult {
            message: "Session revoked successfully".to_string(),
        })),
        Err(e) => Err(e),
    }
}
