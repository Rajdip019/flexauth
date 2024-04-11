use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct NewUser {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub role: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub _id: ObjectId,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEmail {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserPayload {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserRolePayload {
    pub role: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleUserActivationStatusPayload {
    pub is_active: Option<bool>,
    pub email: String,
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
