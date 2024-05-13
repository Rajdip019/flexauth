use crate::{
    core::user::User,
    errors::{Error, Result},
    models::password_model::{
        ForgetPasswordPayload, ForgetPasswordResetPayload, ResetPasswordPayload,
    },
    utils::validation_utils::Validation,
    AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;
use bson::doc;
use serde_json::{json, Value};

#[debug_handler]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    payload: Json<ResetPasswordPayload>,
) -> Result<Json<Value>> {
    // check if payload is valid
    if payload.email.is_empty() | payload.old_password.is_empty() | payload.new_password.is_empty()
    {
        return Err(Error::InvalidPayload {
            message: "Email and password are required.".to_string(),
        });
    }

    if !Validation::password(&payload.new_password) || !Validation::password(&payload.old_password)
    {
        return Err(Error::InvalidPayload {
            message: "Password must be at least 8 characters long.".to_string(),
        });
    }

    match User::change_password(
        &state.mongo_client,
        &payload.email,
        &payload.old_password,
        &payload.new_password,
    )
    .await
    {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password updated successfully. Please login with the new password."
            })));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn forget_password_request_handler(
    State(state): State<AppState>,
    payload: Json<ForgetPasswordPayload>,
) -> Result<Json<Value>> {
    // check if payload.email exists
    if payload.email.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Email is required.".to_string(),
        });
    }

    match User::forget_password_request(&state.mongo_client, &payload.email).await {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password reset request sent successfully. Please check your email."
            })));
        }
        Err(e) => return Err(e),
    }
}

#[debug_handler]
pub async fn forget_password_reset_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    payload: Json<ForgetPasswordResetPayload>,
) -> Result<Json<Value>> {
    // check if payload is valid
    if payload.email.is_empty() | payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Email is required.".to_string(),
        });
    }
    match User::forget_password_reset(&state.mongo_client, &id, &payload.email, &payload.password)
        .await
    {
        Ok(_) => {
            return Ok(Json(json!({
                "message": "Password updated successfully. Please login with the new password."
            })));
        }
        Err(e) => return Err(e),
    }
}
