use axum::{
    extract::State,
    http::{header, HeaderMap},
    Json,
};
use axum_macros::debug_handler;

use crate::{
    core::session::Session,
    errors::{Error, Result},
    models::{
        session_model::{
            DeleteAllSessionsPayload, DeleteAllSessionsResult, DeleteSessionsPayload, DeleteSessionsResult, RevokeAllSessionsPayload, RevokeAllSessionsResult, RevokeSessionsPayload, RevokeSessionsResult, SessionDetailsPayload, SessionRefreshPayload, SessionRefreshResult, SessionResponse, VerifySession
        },
        user_model::UserIdPayload,
    },
    utils::session_utils::IDToken,
    AppState,
};

#[debug_handler]
pub async fn verify_session(
    State(state): State<AppState>,
    payload: Json<VerifySession>,
) -> Result<Json<IDToken>> {
    // check if the token is not empty
    if payload.token.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    // verify the token
    match Session::verify(&state.mongo_client, &payload.token).await {
        Ok(data) => {
            return {
                if data.1 {
                    Ok(Json(data.0))
                } else {
                    Err(Error::SessionExpired {
                        message: "Session expired".to_string(),
                    })
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
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    // verify the token
    match Session::get_all_from_uid(&state.mongo_client, &payload.uid).await {
        Ok(data) => {
            return Ok(Json(data));
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn get_details(
    State(state): State<AppState>,
    payload: Json<SessionDetailsPayload>,
) -> Result<Json<SessionResponse>> {
    // check if the token is not empty
    if payload.uid.is_empty() | payload.session_id.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    match Session::get_details(&state.mongo_client, &payload.uid, &payload.session_id).await {
        Ok(data) => {
            return Ok(Json(data));
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn refresh_session(
    State(state): State<AppState>,
    header: HeaderMap,
    payload: Json<SessionRefreshPayload>,
) -> Result<Json<SessionRefreshResult>> {
    // check if the token is not empty
    if payload.id_token.is_empty()
        || payload.refresh_token.is_empty()
        || payload.session_id.is_empty()
        || payload.uid.is_empty()
    {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
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

    // verify the token
    match Session::refresh(
        &state.mongo_client,
        &payload.uid,
        &payload.session_id,
        &payload.id_token,
        &payload.refresh_token,
        &user_agent,
    )
    .await
    {
        Ok(data) => {
            return Ok(Json(SessionRefreshResult {
                uid: payload.uid.clone(),
                session_id: payload.session_id.clone(),
                id_token: data.0,
                refresh_token: data.1,
            }));
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn revoke(
    State(state): State<AppState>,
    payload: Json<RevokeSessionsPayload>,
) -> Result<Json<RevokeSessionsResult>> {
    // check if the token is not empty
    if payload.session_id.is_empty() | payload.uid.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    // revoke the session
    match Session::revoke(&state.mongo_client, &payload.session_id, &payload.uid).await {
        Ok(_) => {
            return Ok(Json(RevokeSessionsResult {
                message: "Session revoked successfully".to_string(),
            }))
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn revoke_all(
    State(state): State<AppState>,
    payload: Json<RevokeAllSessionsPayload>,
) -> Result<Json<RevokeAllSessionsResult>> {
    // revoke all the sessions
    match Session::revoke_all(&state.mongo_client, &payload.uid).await {
        Ok(_) => {
            return Ok(Json(RevokeAllSessionsResult {
                message: "All sessions revoked successfully".to_string(),
            }))
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn delete(
    State(state): State<AppState>,
    payload: Json<DeleteSessionsPayload>,
) -> Result<Json<DeleteSessionsResult>> {
    // revoke all the sessions
    if payload.session_id.is_empty() | payload.uid.is_empty(){
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    match Session::delete(&state.mongo_client, &payload.session_id, &payload.uid).await {
        Ok(_) => {
            return Ok(Json(DeleteSessionsResult {
                message: "Session deleted successfully".to_string(),
            }))
        }
        Err(e) => return Err(e),
    };
}

#[debug_handler]
pub async fn delete_all(
    State(state): State<AppState>,
    payload: Json<DeleteAllSessionsPayload>,
) -> Result<Json<DeleteAllSessionsResult>> {
    // revoke all the sessions
    if payload.uid.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload passed".to_string(),
        });
    }

    match Session::delete_all(&state.mongo_client, &payload.uid).await {
        Ok(_) => {
            return Ok(Json(DeleteAllSessionsResult {
                message: "All sessions deleted successfully".to_string(),
            }))
        }
        Err(e) => return Err(e),
    };
}
