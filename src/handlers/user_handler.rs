use crate::{
    errors::{Error, Result},
    models::{dek_model::Dek, user_model::{SignInPayload, UpdateUserPayload, User, UserEmail}},
    utils::{encryption_utils::{create_dek, decrypt_data, encrypt_data}, hashing_utils::{salt_and_hash_password, verify_password}},
    AppState,
};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::{doc, uuid, DateTime};
use chrono::Utc;
use mongodb::Collection;
use serde::de::DeserializeOwned;
// use mongodb::Client;
use futures::stream::StreamExt;
use serde_json::{json, Value};
use std::env;

use crate::models::user_model::SignUpPayload;

#[debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    payload: Json<SignUpPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: add_user_handler called");
    // check if the payload is empty
    if payload.name.is_empty()
        || payload.email.is_empty()
        || payload.role.is_empty()
        || payload.password.is_empty()
    {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");

    let collection: Collection<User> = db.collection("users");
    let cursor = collection
        .find_one(
            Some(doc! {
                "email": payload.email.clone()
            }),
            None,
        )
        .await
        .unwrap();

    if cursor.is_some() {
        return Err(Error::UserAlreadyExists {
            message: "User already exists".to_string(),
        });
    }

    // hash and salt the password
    let hashed_and_salted_pass = salt_and_hash_password(&payload.password);

    let key = create_dek(); // create a data encryption key for new user

    // encrypt the password and salt with the dek
    let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &key);
    let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &key);

    // format the password and salt with a delimiter
    let formatted_pass_with_salt = format!("{}.{}", encrypted_password, encrypted_salt);

    // encrypt other sensitive data with the dek
    let encrypted_email = encrypt_data(&payload.email, &key);
    let encrypted_role = encrypt_data(&payload.role, &key);

    // create a new uid for the user
    let uid = uuid::Uuid::new();

    // insert the user in the users collection
    let user = doc! {
        "uid": uid,
        "name": payload.name.clone(),
        "email": encrypted_email.clone(),
        "role": encrypted_role,
        "password": formatted_pass_with_salt,
        "email_verified": false,
        "is_active": true,
        "created_at": Utc::now(),
        "updated_at": Utc::now(),
    };

    db.collection("users")
        .insert_one(user.clone(), None)
        .await
        .unwrap();

    // insert the dek and email kek in the deks collection by encrypting them with the server kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_dek = encrypt_data(&key, &server_kek);
    let encrypted_email_kek = encrypt_data(&key, &server_kek);
    let encrypted_uid = encrypt_data(&uid.to_string(), &server_kek);

    db.collection("deks")
        .insert_one(
            doc! {
                "uid": encrypted_uid,
                "email": encrypted_email_kek.clone(),
                "dek": encrypted_dek,
                "created_at": Utc::now(),
                "updated_at": Utc::now(),
            },
            None,
        )
        .await
        .unwrap();

    let res = Json(json!({
        "message": "Signup successful",
        "user": user,
    }));

    Ok(res)
}

pub async fn signin_handler(
    State(state): State<AppState>,
    payload: Json<SignInPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: signin_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    // encrypt the email using kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    // find the user in the dek collection using the encrypted email
    let db = state.mongo_client.database("test");
    let collection: Collection<Dek> = db.collection("deks");
    let cursor = collection
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();
    
    if cursor.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor.unwrap();

    // decrypt the dek using the server kek
    let dek = decrypt_data(&dek_data.dek, &server_kek);
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    // find the user in the users collection using the uid
    let collection: Collection<User> = db.collection("users");
    let user_cursor = collection
        .find_one(
            Some(doc! {
                "uid": uid.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if user_cursor.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let user = user_cursor.unwrap();

    // decrypt the password and salt using the dek
    let password_hashed = decrypt_data(user.password.split('.').collect::<Vec<&str>>()[0], &dek);
    let salt = decrypt_data(user.password.split('.').collect::<Vec<&str>>()[1], &dek);

    // verify the password
    if verify_password(&payload.password, &salt, &password_hashed) {
        let res = Json(json!({
            "message": "Signin successful",
            "user": user,
        }));

        Ok(res)
    } else {
        Err(Error::UserNotFound {
            message: "User not found".to_string(),
        })
    }
}

trait MongoDbModel: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> &'static str;
    fn db_name() -> &'static str;
}

pub async fn get_all_users_handler(State(state): State<AppState>) -> Result<Json<Value>> {
    println!(">> HANDLER: get_user_handler called");

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut users = Vec::new();
    while let Some(user) = cursor.next().await {
        users.push(user.unwrap());
    }

    let res = Json(json!(users));

    Ok(res)
}

pub async fn update_user_handler(
    State(state): State<AppState>,
    payload: Json<UpdateUserPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: update_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.name.is_empty() || payload.role.is_empty() {
        return Err(Error::UpdateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection
        .update_one(
            doc! {
                "email": payload.email.clone(),
            },
            doc! {
                "$set": {
                    "name": payload.name.clone(),
                    "role": payload.role.clone(),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    let modified_count = cursor.modified_count;

    if modified_count == 0 {
        // send back a 404 to
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let res = Json(json!({
        "message": "User updated",
        "user": *payload,
    }));

    Ok(res)
}

#[debug_handler]
pub async fn get_user_handler(
    State(state): State<AppState>,
    payload: Json<UserEmail>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: get_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection
        .find_one(
            Some(doc! {
                "email": payload.email.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    let user = cursor.unwrap();

    let res = Json(json!({
        "message": "User found",
        "user": user,
    }));

    Ok(res)
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    payload: Json<UserEmail>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: delete_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection
        .delete_one(
            doc! {
                "email": payload.email.clone(),
            },
            None,
        )
        .await
        .unwrap();

    let deleted_count = cursor.deleted_count;

    if deleted_count == 0 {
        // send back a 404 to
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let res = Json(json!({
        "message": "User Deleted",
        "delete_count": deleted_count,
    }));

    Ok(res)
}
