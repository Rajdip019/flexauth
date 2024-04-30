use std::env;

use axum::Json;
use bson::{doc, oid::ObjectId, uuid, DateTime};
use mongodb::{Client, Collection};
use serde_json::{json, Value};

use crate::{
    core::user::User,
    errors::{Error, Result},
    models::auth_model::{SignInPayload, SignUpPayload},
    traits::decryption::Decrypt,
    utils::{
        encryption_utils::{add_dek_to_db, create_dek, encrypt_data},
        hashing_utils::verify_password,
        session_utils::sign_jwt,
        user_utils::get_user_dek,
    },
};

pub async fn sign_up(mongo_client: &Client, payload: Json<SignUpPayload>) -> Result<Json<Value>> {
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

    let db = mongo_client.database("test");

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

    let dek = create_dek(); // create a data encryption key for new user

    // create a new uid for the user
    let uid = uuid::Uuid::new();

    let user = User {
        _id: ObjectId::new(),
        uid: uid.to_string(),
        name: payload.name.clone(),
        email: payload.email.clone(),
        role: payload.role.clone(),
        password: payload.password.clone(),
        email_verified: false,
        is_active: true,
        created_at: Some(DateTime::now()),
        updated_at: Some(DateTime::now()),
    };

    User::encrypt_and_add(&user, mongo_client).await.unwrap();

    // insert the dek and email kek in the deks collection by encrypting them with the server kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_dek = encrypt_data(&dek, &server_kek);
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);
    let encrypted_uid = encrypt_data(&uid.to_string(), &server_kek);

    let _ = add_dek_to_db(
        &encrypted_email_kek,
        &encrypted_uid,
        &encrypted_dek,
        mongo_client,
    )
    .await
    .unwrap();

    // issue a jwt token
    let token = match sign_jwt(&user, &dek) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Error signing jwt token: {}", err);
            return Err(Error::IdTokenCreationError {
                message: err.to_string(),
            });
        }
    };

    let res = Json(json!({
        "message": "Signup successful",
        "user": {
            "name": payload.name,
            "email": payload.email,
            "role": payload.role,
            "created_at": DateTime::now(),
            "updated_at": DateTime::now(),
            "email_verified": false,
            "is_active": true,
            "uid": uid.to_string(),
            "token": token,
        }
    }));

    match user.encrypt_and_add(&mongo_client).await {
        Ok(_) => return Ok(res),
        Err(e) => return Err(e),
    };
}

pub async fn sign_in(mongo_client: &Client, payload: Json<SignInPayload>) -> Result<Json<Value>> {
    println!(">> HANDLER: signin_handler called");

    // check if the payload is empty
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }
    let db = mongo_client.database("test");

    let dek_data = match get_user_dek(&mongo_client, &payload.email).await {
        Ok(dek) => dek,
        Err(e) => return Err(e),
    };

    let encrypted_uid = encrypt_data(&dek_data.uid, &dek_data.dek);
    // find the user in the users collection using the uid
    let collection: Collection<User> = db.collection("users");
    let cursor = collection
        .find_one(
            Some(doc! {
                "uid": encrypted_uid
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

    let mut user: User = cursor.unwrap();
    let decrypted_user = user.decrypt(&dek_data.dek);

    // verify the password
    if verify_password(&payload.password, &decrypted_user.password) {
        // issue a jwt token
        let token = match sign_jwt(&user, &dek_data.dek) {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Error signing jwt token: {}", err);
                return Err(Error::IdTokenCreationError {
                    message: err.to_string(),
                });
            }
        };
        let res = Json(json!({
            "message": "Signin successful",
            "user": {
                "uid": decrypted_user.uid,
                "name": decrypted_user.name,
                "email": decrypted_user.email,
                "role": decrypted_user.role,
                "created_at": decrypted_user.created_at,
                "updated_at": decrypted_user.updated_at,
                "email_verified": decrypted_user.email_verified,
                "is_active": decrypted_user.is_active,
                "token": token,
            },
        }));

        Ok(res)
    } else {
        Err(Error::UserNotFound {
            message: "User not found".to_string(),
        })
    }
}
