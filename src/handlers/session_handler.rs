use axum::{extract::State, Json};
use axum_macros::debug_handler;

use crate::{core::session::Session, errors::{Error, Result}, models::{session_model::{SessionResponse, VerifySession}, user_model::UserIdPayload}, utils::session_utils::IDToken, AppState};

#[debug_handler]
pub async fn verify_session(
    State(state): State<AppState>,
    payload: Json<VerifySession>,
) -> Result<Json<IDToken>> {
    // check if the token is not empty
    if payload.token.is_empty() {
        return Err(Error::InvalidPayload { message: "Invalid payload passed".to_string() });
    }

    // verify the token
    match Session::verify(&state.mongo_client, &payload.token).await {
        Ok(data) => {
            return {
                if data.1 {
                    Ok(Json(data.0))
                } else {
                    Err(Error::SessionExpired { message: "Session expired".to_string() })
                }
            }
        }
        Err(e) => return Err(e),
        
    };
}

#[debug_handler]
pub async fn get_all_from_uid(
    State(state): State<AppState>,
    payload: Json<UserIdPayload>,
) -> Result<Json<Vec<SessionResponse>>> {
    // check if the token is not empty
    if payload.uid.is_empty() {
        return Err(Error::InvalidPayload { message: "Invalid payload passed".to_string() });
    }

    // verify the token
    match Session::get_all_from_uid(&state.mongo_client, &payload.uid).await {
        Ok(data) => {
            return Ok(Json(data));
        }
        Err(e) => return Err(e),
    };
}