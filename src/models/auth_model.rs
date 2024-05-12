use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignInPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct SessionResponseForSignInOrSignUp {
    pub session_id: String,
    pub id_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignInOrSignUpResponse {
    pub message: String,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub email_verified: bool,
    pub is_active: bool,
    pub session: SessionResponseForSignInOrSignUp,
}
