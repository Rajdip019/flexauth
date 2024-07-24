use core::str;

use bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::core::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserEmailPayload {
    pub email: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserEmailResponse {
    pub message: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserIdPayload {
    pub uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId {
    pub uid: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateUserPayload {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserResponse {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateUserRolePayload {
    pub role: String,
    pub email: String,
}
#[derive(Serialize, Debug, Clone)]
pub struct UpdateUserRoleResponse {
    pub message: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ToggleUserActivationStatusPayload {
    pub is_active: Option<bool>,
    pub email: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ToggleUserActivationStatusResponse {
    pub message: String,
    pub email: String,
    pub is_active: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub uid: String,
    pub name: String,
    pub role: String,
    pub email: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecentUserPayload {
    pub limit: i64,
}