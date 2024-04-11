use bson::DateTime;

use crate::models::dek_model::NewDek;

pub fn new_dek(uid: String, email: String, dek: String) -> NewDek {
    NewDek {
        uid,
        email,
        dek,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}
