use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug, Clone, Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub fn new_user(name: String, email: String, role: String) -> NewUser {
    NewUser {
        name,
        email,
        role,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}

#[derive(Serialize, Deserialize,Debug, Clone, Default)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub role: String,
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
