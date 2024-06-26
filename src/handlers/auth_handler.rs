use axum::{
    extract::State,
    http::{header, HeaderMap},
    Json,
};
use axum_macros::debug_handler;

use crate::{
    core::{auth::Auth, session::Session, user::User},
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
    header: HeaderMap,
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
        return Err(Error::InvalidEmail {
            message: "Invalid Email".to_string(),
        });
    }

    let user = User::get_from_email(&state.mongo_client, &payload.email).await;
    if user.is_ok() {
        return Err(Error::UserAlreadyExists {
            message: "User already exists".to_string(),
        });
    }

    if !Validation::password(&payload.password) {
        return Err(Error::InvalidPassword {
            message: "The password must contain at least one alphabetic character (uppercase or lowercase), at least one digit, and must be at least 8 characters long.".to_string(),
        });
    }

    // get user-agent form the header
    let user_agent = match header.get(header::USER_AGENT) {
        Some(ua) => ua.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    if user_agent.is_empty() {
        return Err(Error::InvalidUserAgent {
            message: "Invalid User Agent, Can't let random user to signin".to_string(),
        });
    }

    match Auth::sign_up(
        &state.mongo_client,
        &payload.name,
        &payload.email,
        &payload.role,
        &payload.password,
        &user_agent,
    )
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn signin_handler(
    State(state): State<AppState>,
    header: HeaderMap,
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
        return Err(Error::InvalidEmail {
            message: "Invalid Email".to_string(),
        });
    }

    // get user-agent form the header
    let user_agent = match header.get(header::USER_AGENT) {
        Some(ua) => ua.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    if user_agent.is_empty() {
        return Err(Error::InvalidUserAgent {
            message: "Invalid User Agent, Can't let random user to signin".to_string(),
        });
    }

    match Auth::sign_in(
        &state.mongo_client,
        &payload.email,
        &payload.password,
        &user_agent,
    )
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn signout_handler(
    State(state): State<AppState>,
    payload: Json<RevokeSessionsPayload>,
) -> Result<Json<RevokeSessionsResult>> {
    println!(">> HANDLER: signout_handler called");

    if payload.session_id.is_empty() | payload.uid.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    match Session::revoke(&state.mongo_client, &payload.session_id, &payload.uid).await {
        Ok(_) => Ok(Json(RevokeSessionsResult {
            message: "Session revoked successfully".to_string(),
        })),
        Err(e) => Err(e),
    }
}
