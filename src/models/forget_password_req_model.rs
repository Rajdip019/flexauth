use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ForgetPasswordRequest {
    _id: ObjectId,
    pub email: String,
    pub id: String,
    pub is_used: bool,
    pub valid_till: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct NewForgetPasswordRequest {
    pub id: String,
    pub email: String,
    pub is_used: bool,
    pub valid_till: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ForgetPasswordResetPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetPasswordRequestPayload {
    pub email: String,
}