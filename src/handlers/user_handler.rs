use crate::{
    errors::{Error, Result},
    models::{
        dek_model::Dek,
        user_model::{
            ToggleUserActivationStatusPayload, UpdateUserPayload, UpdateUserRolePayload, User,
            UserEmail,
        },
    },
    utils::{
        encryption_utils::{decrypt_data, decrypted_user, encrypt_data},
        user_utils::{delete_user, get_all_users, toggle_user_activation, update_user_role},
    },
    AppState,
};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::{doc, DateTime};
use mongodb::Collection;
use serde::de::DeserializeOwned;
// use mongodb::Client;
use serde_json::{json, Value};
use std::env;

trait MongoDbModel: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> &'static str;
    fn db_name() -> &'static str;
}

pub async fn get_all_users_handler(State(state): State<AppState>) -> Result<Json<Value>> {
    println!(">> HANDLER: get_user_handler called");

    let res = get_all_users(State(state)).await.unwrap();

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
            message: "DEK not found".to_string(),
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

    let res = update_user_role(State(state), payload.email.clone(), payload.role.clone())
        .await
        .unwrap();

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

    let res = toggle_user_activation(
        State(state),
        payload.email.clone(),
        payload.is_active.unwrap(),
    )
    .await
    .unwrap();

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

    let user = decrypted_user(&user_data, &dek);

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

    let res = delete_user(State(state), payload.email.clone())
        .await
        .unwrap();

    Ok(res)
}
