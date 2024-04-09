use bson::{uuid, DateTime};

use crate::models::user_model::NewUser;

pub fn new_user(name: String, email: String, role: String, password: String) -> NewUser {
    NewUser {
        uid: uuid::Uuid::new().to_string(),
        name,
        email,
        role,
        password,
        email_verified: false,
        is_active: true,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}
