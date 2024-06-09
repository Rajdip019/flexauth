use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct VerifySession {
    pub token: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SessionResponse {
    pub uid : String,
    pub session_id : String,
    pub email : String,
    pub user_agent : String,
    pub is_revoked : bool,
    pub created_at : DateTime,
    pub updated_at : DateTime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionRefreshPayload {
    pub uid: String,
    pub session_id: String,
    pub id_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct SessionRefreshResult {
    pub uid: String,
    pub session_id: String,
    pub id_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RevokeAllSessionsPayload {
    pub uid: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct RevokeAllSessionsResult {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RevokeSessionsPayload {
    pub session_id: String,
    pub uid: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct RevokeSessionsResult {
    pub message: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct DeleteAllSessionsPayload {
    pub uid: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct DeleteAllSessionsResult {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeleteSessionsPayload {
    pub session_id: String,
    pub uid: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct DeleteSessionsResult {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionDetailsPayload {
    pub uid: String,
    pub session_id: String,
}