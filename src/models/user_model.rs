use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub role: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AddUserPayload {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub fn new_user(name: String, email: String, role: String, password: String) -> NewUser {
    NewUser {
        name,
        email,
        password,
        role,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        email_verified: false,
        is_active: false,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
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
pub struct UserUpdate {
    pub name: String,
    pub email: String,
    pub role: String,
}
