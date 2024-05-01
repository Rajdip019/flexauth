use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ForgetPasswordRequest {
    pub _id: ObjectId,
    pub email: String,
    pub id: String,
    pub is_used: bool,
    pub valid_till: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResetPasswordPayload {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ForgetPasswordResetPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetPasswordPayload {
    pub email: String,
}