use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct VerifySession {
    pub token: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SessionResponse {
    pub uid : String,
    pub email : String,
    pub user_agent : String,
    pub is_revoked : bool,
    pub created_at : DateTime,
    pub updated_at : DateTime,
}