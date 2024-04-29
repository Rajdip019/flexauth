use crate::{
    errors::{Error, Result},
    models::{
        dek_model::Dek,
        user_model::{
            ToggleUserActivationStatusPayload, UpdateUserPayload, UpdateUserRolePayload, User,
            UserEmail,
        },
    },
    utils::encryption_utils::{decrypt_data, encrypt_data},
    AppState,
};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::{doc, DateTime};
use mongodb::Collection;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::env;

trait MongoDbModel: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> &'static str;
    fn db_name() -> &'static str;
}

pub async fn get_all_users_handler(State(state): State<AppState>) -> Result<Json<Value>> {
    println!(">> HANDLER: get_user_handler called");

    let res = User::get_all_users(&state.mongo_client).await.unwrap();

    Ok(Json(json!({
        "users": res,
    })))
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

    let res = User::update_user_role(&State(state).mongo_client, &payload.email, &payload.role)
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

    let res = User::toggle_user_activation(
        &State(state).mongo_client,
        &payload.email,
        &payload.is_active.unwrap(),
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

    let user = User::get_user_from_email(&state.mongo_client, &payload.email)
        .await
        .unwrap();

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

    let res = User::delete_user(&State(state).mongo_client, &payload.email)
        .await
        .unwrap();

    Ok(res)
}
