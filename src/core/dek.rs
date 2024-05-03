use std::env;

use aes_gcm::{aead::OsRng, AeadCore, Aes256Gcm, KeyInit};
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use crate::{
    errors::{Error, Result},
    traits::{decryption::Decrypt, encryption::Encrypt}, utils::encryption_utils::Encryption,
};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Dek {
    pub _id: ObjectId,
    pub uid: String,
    pub email: String,
    pub dek: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Dek {
    pub fn new(uid: &str, email: &str, dek: &str) -> Self {
        Self {
            _id: ObjectId::new(),
            uid: uid.to_string(),
            email: email.to_string(),
            dek: dek.to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn generate() -> String {
        let key = Aes256Gcm::generate_key(OsRng);
        // convert the key to hex string
        let hex_key = key
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
            .chars()
            .take(32)
            .collect::<String>();
        let iv = Aes256Gcm::generate_nonce(&mut OsRng);
        // convert the iv to hex string
        let hex_iv = iv
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
            .chars()
            .take(12)
            .collect::<String>();
        // connect the key and iv with . between them
        let key_iv = format!("{}.{}", hex_key, hex_iv);
        return key_iv;
    }
    
    pub async fn encrypt_and_add(&self, mongo_client: &Client) -> Result<Self> {
        let db = mongo_client.database("test");
        let collection_dek: Collection<Dek> = db.collection("deks");

        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        let encrypted_dek = self.encrypt(&server_kek);

        match collection_dek.insert_one(&encrypted_dek, None).await {
            Ok(_) => return Ok(self.clone()),
            Err(e) => return Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn get(mongo_client: &Client, identifier: &str) -> Result<Self> {
        let db = mongo_client.database("test");
        let collection_dek: Collection<Dek> = db.collection("deks");

        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // check if the identifier is a email or uid using regex
        let email_regex =
            regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        let is_email = email_regex.is_match(identifier);
        match is_email {
            true => {
                // encrypt the email using kek
                let encrypted_email_kek = Encryption::encrypt_data(&identifier, &server_kek);
                let cursor_dek = collection_dek
                    .find_one(
                        Some(doc! {
                            "email": encrypted_email_kek.clone(),
                        }),
                        None,
                    )
                    .await
                    .unwrap();

                match cursor_dek {
                    Some(data) => return Ok(data.decrypt(&server_kek)),
                    None => {
                        return Err(Error::KeyNotFound {
                            message: "DEK not found".to_string(),
                        });
                    }
                };
            }
            false => {
                // encrypt the uid using kek
                let encrypted_uid_kek = Encryption::encrypt_data(&identifier, &server_kek);
                let cursor_dek = collection_dek
                    .find_one(
                        Some(doc! {
                            "uid": encrypted_uid_kek.clone(),
                        }),
                        None,
                    )
                    .await
                    .unwrap();

                match cursor_dek {
                    Some(data) => return Ok(data.decrypt(&server_kek)),
                    None => {
                        return Err(Error::KeyNotFound {
                            message: "DEK not found".to_string(),
                        });
                    }
                };
            }
        }
    }
}
