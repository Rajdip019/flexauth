use crate::{
    errors::{Error, Result},
    models::{
        dek_model::Dek,
        user_model::{
            SignInPayload, ToggleUserActivationStatusPayload, UpdateUserPayload,
            UpdateUserRolePayload, User, UserEmail, UserResponse,
        },
    },
    utils::{
        encryption_utils::{create_dek, decrypt_data, encrypt_data},
        hashing_utils::{salt_and_hash_password, verify_password},
    },
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
        return Err(Error::InvalidPayload {
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
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);
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
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    // encrypt the email using kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    println!(">> Encrypted email kek: {:?}", encrypted_email_kek);

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
            "user": {
                "name": user.name,
                "email": decrypt_data(&user.email, &dek),
                "role": decrypt_data(&user.role, &dek),
                "created_at": user.created_at,
                "updated_at": user.updated_at,
                "email_verified": user.email_verified,
                "is_active": user.is_active,
                "uid": user.uid,
            },
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
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // let mut cursor = collection.find(None, None).await.unwrap();
    let mut cursor_dek = collection_dek.find(None, None).await.unwrap();

    let mut users = Vec::new();

    // iterate over the users and decrypt the data
    while let Some(dek) = cursor_dek.next().await {
        let dek_data = dek.unwrap();

        // decrypt the email & DEK using the server kek
        let decrypted_email = decrypt_data(&dek_data.email, &server_kek);
        let dek = decrypt_data(&dek_data.dek, &server_kek);

        // Encrypt the email with the DEK
        let encrypted_email_kek = encrypt_data(&decrypted_email, &dek);

        // find the user in the users collection using the encrypted email to iterate over the users
        let cursor_user = collection
            .find_one(
                Some(doc! {
                    "email": encrypted_email_kek,
                }),
                None,
            )
            .await
            .unwrap();

        if cursor_user.is_none() {
            return Err(Error::UserNotFound {
                message: "User not found".to_string(),
            });
        }

        let user_data = cursor_user.unwrap();

        users.push(UserResponse {
            name: user_data.name,
            email: decrypt_data(&user_data.email, &dek),
            role: decrypt_data(&user_data.role, &dek),
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
            email_verified: user_data.email_verified,
            is_active: user_data.is_active,
            uid: user_data.uid,
        });
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
    if payload.email.is_empty() || payload.name.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    let cursor_dek = collection_dek
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if cursor_dek.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor_dek.unwrap();

    // decrypt the dek using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    // find the user in the users collection using the uid
    let cursor = collection
        .update_one(
            doc! {
                "uid": uid,
            },
            doc! {
                "$set": {
                    "name": payload.name.clone(),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    let modified_count = cursor.modified_count;

    // Return Error if User is not there
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

pub async fn update_user_role_handler(
    State(state): State<AppState>,
    payload: Json<UpdateUserRolePayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: update_user_role_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.role.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    let cursor_dek = collection_dek
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if cursor_dek.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor_dek.unwrap();

    // decrypt the dek using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);
    let dek = decrypt_data(&dek_data.dek, &server_kek);

    let encrypted_role = encrypt_data(&payload.role, &dek);

    // find the user in the users collection using the uid
    let cursor = collection
        .update_one(
            doc! {
                "uid": uid,
            },
            doc! {
                "$set": {
                    "role": encrypted_role,
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    let modified_count = cursor.modified_count;

    // Return Error if User is not there
    if modified_count == 0 {
        // send back a 404 to
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let res = Json(json!({
        "message": "User Role updated",
        "user": *payload,
    }));

    Ok(res)
}

pub async fn toggle_user_activation_status(
    State(state): State<AppState>,
    payload: Json<ToggleUserActivationStatusPayload>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: update_user_role_handler called");

    match payload.is_active {
        Some(_) => {
            if payload.email.is_empty() {
                return Err(Error::InvalidPayload {
                    message: "Invalid payload".to_string(),
                });
            }
        }
        None => {
            return Err(Error::InvalidPayload {
                message: "Invalid payload".to_string(),
            });
        }
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    let cursor_dek = collection_dek
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if cursor_dek.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor_dek.unwrap();

    // decrypt the dek using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    // find the user in the users collection using the uid
    println!(">> isActive: {:?}", payload.is_active);
    let cursor = collection
        .update_one(
            doc! {
                "uid": uid,
            },
            doc! {
                "$set": {
                    "is_active": payload.is_active.unwrap(),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    let modified_count = cursor.modified_count;

    // Return Error if User is not there
    if modified_count == 0 {
        // send back a 404 to
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let res = Json(json!({
        "message": "User Activation Status updated",
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
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }
    let db = state.mongo_client.database("test");
    let collection_user: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    let cursor_dek = collection_dek
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if cursor_dek.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor_dek.unwrap();

    // decrypt the dek using the server kek
    let dek = decrypt_data(&dek_data.dek, &server_kek);
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    // find the user in the users collection using the uid
    let user_cursor = collection_user
        .find_one(
            Some(doc! {
                "uid": uid.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    // Return Error if User is not there
    if user_cursor.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let user_data = user_cursor.unwrap();

    let user = UserResponse {
        name: user_data.name,
        email: decrypt_data(&user_data.email, &dek),
        role: decrypt_data(&user_data.role, &dek),
        created_at: user_data.created_at,
        updated_at: user_data.updated_at,
        email_verified: user_data.email_verified,
        is_active: user_data.is_active,
        uid: user_data.uid,
    };

    Ok(Json(json!({
        "message": "User found",
        "user": user,
    })))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    payload: Json<UserEmail>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: delete_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);

    let cursor_dek = collection_dek
        .find_one(
            Some(doc! {
                "email": encrypted_email_kek.clone(),
            }),
            None,
        )
        .await
        .unwrap();

    if cursor_dek.is_none() {
        return Err(Error::UserNotFound {
            message: "User not found".to_string(),
        });
    }

    let dek_data = cursor_dek.unwrap();

    // decrypt the uid using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    let cursor = collection
        .delete_one(
            doc! {
                "uid": uid,
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

    // delete the dek from the deks collection
    let cursor_dek_delete = collection_dek
        .delete_one(
            doc! {
                "uid": dek_data.uid,
            },
            None,
        )
        .await
        .unwrap();

    let deleted_count_dek = cursor_dek_delete.deleted_count;

    if deleted_count_dek == 0 {
        // send back a 404 to
        return Err(Error::UserNotFound {
            message: "DEK not found".to_string(),
        });
    }

    let res = Json(json!({
        "message": "User Deleted",
        "delete_count": deleted_count,
    }));

    Ok(res)
}
