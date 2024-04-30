use std::env;

use bson::{doc, DateTime};

use crate::{
    errors::{Error, Result},
    models::dek_model::{Dek, NewDek},
};

use super::encryption_utils::{decrypt_data, encrypt_data};

impl Dek {
    pub fn new_dek(uid: &str, email: &str, dek: &str) -> NewDek {
        NewDek {
            uid: uid.to_string(),
            email: email.to_string(),
            dek: dek.to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn get_dek_from_email(mongo_client: &mongodb::Client, email: &str) -> Result<String> {
        let db = mongo_client.database("test");
        let collection = db.collection("deks");

        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // encrypt the email using kek
        let encrypted_email_kek = encrypt_data(&email, &server_kek);

        let cursor_dek = collection
            .find_one(
                Some(doc! {
                    "email": encrypted_email_kek
                }),
                None,
            )
            .await
            .unwrap();

        if cursor_dek.is_none() {
            return Err(Error::UserNotFound {
                message: "DEK not found".to_string(),
            });
        }

        let dek_data: Dek = cursor_dek.unwrap();

        let dek = decrypt_data(&dek_data.dek, &server_kek);

        Ok(dek)
    }

    pub async fn get_dek_from_uid(mongo_client: &mongodb::Client, uid: &str) -> Result<String> {
        let db = mongo_client.database("test");
        let collection = db.collection("deks");

        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // encrypt the email using kek
        let encrypted_uid_kek = encrypt_data(&uid, &server_kek);

        let cursor_dek = collection
            .find_one(
                Some(doc! {
                    "uid": encrypted_uid_kek
                }),
                None,
            )
            .await
            .unwrap();

        if cursor_dek.is_none() {
            return Err(Error::UserNotFound {
                message: "DEK not found".to_string(),
            });
        }

        let dek_data: Dek = cursor_dek.unwrap();

        let dek = decrypt_data(&dek_data.dek, &server_kek);

        Ok(dek)
    }
}