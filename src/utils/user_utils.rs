use bson::DateTime;

use crate::models::user_model::NewUser;

pub fn new_user(name: String, email: String, role: String) -> NewUser {
    NewUser {
        name,
        email,
        role,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}