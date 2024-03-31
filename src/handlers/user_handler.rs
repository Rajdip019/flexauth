use crate::{
    error::{Error, Result},
    models::user_model::{User, UserEmail, UserUpdate},
    AppState,
};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::{doc, DateTime};
use chrono::Utc;
use mongodb::Collection;
use serde::de::DeserializeOwned;
// use mongodb::Client;
use futures::stream::StreamExt;
use serde_json::{json, Value};

use crate::models::user_model::NewUser;

#[debug_handler]
pub async fn add_user_handler(
    State(state): State<AppState>,
    payload: Json<NewUser>,
) -> Result<Json<Value>> {
    println!(">> HANDLER: add_user_handler called");
    // check if the payload is empty
    if payload.name.is_empty() || payload.email.is_empty() || payload.role.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let user = doc! {
        "name": payload.name.clone(),
        "email": payload.email.clone(),
        "role": payload.role.clone(),
        "created_at": Utc::now(),
        "updated_at": Utc::now(),
    };

    state
        .mongo_client
        .database("test")
        .collection("users")
        .insert_one(user.clone(), None)
        .await
        .unwrap();

    let res = Json(json!({
        "message": "User added successfully",
        "user": user,
    }));

    Ok(res)
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

pub async fn update_user_handler(State(state): State<AppState>, payload: Json<UserUpdate>) -> Result<Json<Value>> {
    println!(">> HANDLER: update_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection.update_one(
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
    ).await.unwrap();

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
pub async fn get_user_handler(State(state): State<AppState>, payload: Json<UserEmail>) -> Result<Json<Value>> {
    println!(">> HANDLER: get_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection.find_one(
        Some(doc! {
            "email": payload.email.clone(),
        }),
        None,
    ).await.unwrap();

    let user = cursor.unwrap();

    let res = Json(json!({
        "message": "User found",
        "user": user,
    }));

    Ok(res)
}

pub async fn delete_user_handler(State(state): State<AppState>, payload: Json<UserEmail>) -> Result<Json<Value>> {
    println!(">> HANDLER: delete_user_handler called");

    // check if the payload is empty
    if payload.email.is_empty() {
        return Err(Error::CreateUserInvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection.delete_one(
        doc! {
            "email": payload.email.clone(),
        },
        None,
    ).await.unwrap();

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
