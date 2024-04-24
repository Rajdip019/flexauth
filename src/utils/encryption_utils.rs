use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};

use axum::{extract::State, Json};
use bson::{doc, oid::ObjectId, DateTime};
use chrono::Utc;
use serde_json::{json, Value};

use crate::{
    errors::Result,
    models::user_model::{User, UserResponse},
    AppState,
};

use super::hashing_utils::salt_and_hash_password;

pub fn create_dek() -> String {
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

pub fn encrypt_data(data: &str, key_iv: &str) -> String {
    // split the key_iv into key and iv
    let key_iv_vec: Vec<&str> = key_iv.split('.').collect();
    let key_buff = Key::<Aes256Gcm>::from_slice(key_iv_vec[0].as_bytes().try_into().unwrap());
    let cipher = Aes256Gcm::new(key_buff);
    let cipher_text = cipher
        .encrypt(key_iv_vec[1].as_bytes().into(), data.as_ref())
        .unwrap();
    // convert the cipher_text to string
    return hex::encode(cipher_text);
}

pub fn decrypt_data(cipher_text: &str, key_iv: &str) -> String {
    // convert the cipher_text to bytes
    let cipher_text = hex::decode(cipher_text).unwrap();
    // split the key_iv into key and iv
    let key_iv_vec: Vec<&str> = key_iv.split('.').collect();
    let key_buff = Key::<Aes256Gcm>::from_slice(key_iv_vec[0].as_bytes().try_into().unwrap());
    let cipher = Aes256Gcm::new(key_buff);
    let data = cipher
        .decrypt(key_iv_vec[1].as_bytes().into(), cipher_text.as_ref())
        .unwrap();
    return String::from_utf8(data).unwrap();
}

pub fn encrypt_user(user: &User, dek: &str) -> User {
    // hash and salt the password
    let hashed_and_salted_pass = salt_and_hash_password(user.password.as_str());

    // encrypt the password and salt with the dek
    let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &dek);
    let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &dek);

    // format the password and salt with a delimiter
    let formatted_pass_with_salt = format!("{}.{}", encrypted_password, encrypted_salt);

    // encrypt other sensitive data with the dek
    let encrypted_email = encrypt_data(&user.email, &dek);
    let encrypted_role = encrypt_data(&user.role, &dek);

    let user = User {
        _id: ObjectId::new(),
        name: user.name.clone(),
        email: encrypted_email.clone(),
        role: encrypted_role.clone(),
        password: formatted_pass_with_salt.clone(),
        created_at: Some(DateTime::now()),
        updated_at: Some(DateTime::now()),
        email_verified: false,
        is_active: true,
        uid: user.uid.to_string(),
    };

    user
}

pub fn decrypted_user(user: &User, dek: &str) -> UserResponse {
    let decrypted_user = UserResponse {
        name: user.name.clone(),
        email: decrypt_data(&user.email, &dek),
        role: decrypt_data(&user.role, &dek),
        created_at: user.created_at,
        updated_at: user.updated_at,
        email_verified: user.email_verified,
        is_active: user.is_active,
        uid: user.uid.clone(),
    };
    decrypted_user
}

pub async fn add_dek_to_db(
    encrypted_email: String,
    encrypted_uid: String,
    encrypted_dek: String,
    State(state): State<AppState>,
) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");

    db.collection("deks")
        .insert_one(
            doc! {
                "uid": encrypted_uid,
                "email": encrypted_email,
                "dek": encrypted_dek,
                "created_at": Utc::now(),
                "updated_at": Utc::now(),
            },
            None,
        )
        .await
        .unwrap();

    let res = Json(json!({
        "message": "DEK added successfully"
    }));

    Ok(res)
}
