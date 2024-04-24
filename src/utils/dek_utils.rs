use std::env;

use axum::{extract::State, Json};
use bson::{doc, DateTime};
use serde_json::{json, Value};

use crate::{
    errors::{Error, Result},
    models::dek_model::{Dek, NewDek},
    AppState,
};

use super::encryption_utils::{decrypt_data, encrypt_data};

pub fn new_dek(uid: String, email: String, dek: String) -> NewDek {
    NewDek {
        uid,
        email,
        dek,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}

pub async fn get_dek_from_email(
    State(state): State<AppState>,
    email: String,
) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
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

    let res = Json(json!({
        "dek": dek
    }));

    Ok(res)
}

pub async fn get_dek_from_uid(State(state): State<AppState>, uid: String) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
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

    let res = Json(json!({
        "dek": dek
    }));

    Ok(res)
}
