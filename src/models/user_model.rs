use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug, Clone, Serialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize,Debug, Clone, Default)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEmail {
    pub email: String,
}
