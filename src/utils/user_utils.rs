use std::env;

use crate::{
    errors::{Error, Result},
    models::{
        dek_model::Dek,
        user_model::{User, UserId, UserResponse},
    },
};
use axum::{extract::State, Json};
use bson::{doc, uuid, DateTime};
use futures::StreamExt;
use mongodb::Collection;
use serde_json::{json, Value};

use crate::{
    models::user_model::{NewUser, UserEmail},
    AppState,
};

use super::encryption_utils::{decrypt_data, encrypt_data};

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

pub async fn get_user_from_email(
    State(state): State<AppState>,
    payload: Json<UserEmail>,
) -> Result<Json<Value>> {
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

    // decrypt the dek using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);

    let cursor_user = collection
        .find_one(
            doc! {
                "uid": uid,
            },
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

    let res = Json(json!({
        "user": user_data,
    }));

    Ok(res)
}

pub async fn get_user_from_uid(
    State(state): State<AppState>,
    payload: Json<UserId>,
) -> Result<Json<Value>> {
    // check if the payload is empty
    if payload.uid.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");

    let cursor_user = collection
        .find_one(
            doc! {
                "uid": payload.uid.clone(),
            },
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

    let res = Json(json!({
        "user": user_data,
    }));

    Ok(res)
}

pub async fn add_user_to_db(State(state): State<AppState>, user: User) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    collection.insert_one(&user, None).await.unwrap();
    let res = Json(json!({
        "message": "User added successfully",
    }));
    Ok(res)
}

pub async fn get_all_users(State(state): State<AppState>) -> Result<Json<Value>> {
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

pub async fn update_user_role(
    State(state): State<AppState>,
    email: String,
    role: String,
) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&email, &server_kek);

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

    let dek_data: Dek = cursor_dek.unwrap();

    // decrypt the dek using the server kek
    let uid = decrypt_data(&dek_data.uid, &server_kek);
    let dek = decrypt_data(&dek_data.dek, &server_kek);

    let encrypted_role = encrypt_data(&role, &dek);

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
        "user": {
            "email": email,
            "role": role,
        },
    }));

    Ok(res)
}

pub async fn toggle_user_activation(
    State(state): State<AppState>,
    email: String,
    is_active: bool,
) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&email, &server_kek);

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
                    "is_active": is_active,
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
        "user": {
            "email": email,
            "is_active": is_active,
        },
    }));

    Ok(res)
}

pub async fn delete_user(State(state): State<AppState>, email: String) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let collection_dek: Collection<Dek> = db.collection("deks");

    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

    // encrypt the email using kek
    let encrypted_email_kek = encrypt_data(&email, &server_kek);

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
