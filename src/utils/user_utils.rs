use std::env;

use bson::doc;
use mongodb::{Client, Collection};

use crate::{errors::{Error, Result}, models::dek_model::Dek, traits::decryption::Decrypt, utils::encryption_utils::encrypt_data};

pub async fn get_user_dek(mongo_client: &Client, identifier: &str) -> Result<Dek> {
    let db = mongo_client.database("test");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // check if the identifier is a email or uid using regex
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    let is_email = email_regex.is_match(identifier);
    match is_email {
        true => {
            // encrypt the email using kek
            let encrypted_email_kek = encrypt_data(&identifier, &server_kek);
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
                Some(mut data) => return Ok(data.decrypt(&server_kek)),
                None => {
                    return Err(Error::KeyNotFound {
                        message: "DEK not found".to_string(),
                    });
                }
            };
        }
        false => {
            // encrypt the uid using kek
            let encrypted_uid_kek = encrypt_data(&identifier, &server_kek);
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
                Some(mut data) => return Ok(data.decrypt(&server_kek)),
                None => {
                    return Err(Error::KeyNotFound {
                        message: "DEK not found".to_string(),
                    });
                }
            };
        }
    }
}