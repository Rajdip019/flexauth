use std::env;

use axum::{extract::State, Json};
use axum_macros::debug_handler;
use bson::{doc, oid::ObjectId, uuid, DateTime};
use mongodb::Collection;
use serde_json::{json, Value};

use crate::{
    errors::{Error, Result},
    models::{
        auth_model::{SignInPayload, SignUpPayload},
        dek_model::Dek,
        user_model::User,
    },
    utils::{
        encryption_utils::{add_dek_to_db, create_dek, decrypt_data, encrypt_data, encrypt_user},
        hashing_utils::verify_password,
        session_utils::sign_jwt,
    },
    AppState,
};

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

    // insert the user in the users collection
    let encrypted_user = encrypt_user(&user, &dek);

    let collection: Collection<User> = db.collection("users");
    collection.insert_one(&encrypted_user, None).await.unwrap();

    // insert the dek and email kek in the deks collection by encrypting them with the server kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_dek = encrypt_data(&dek, &server_kek);
    let encrypted_email_kek = encrypt_data(&payload.email, &server_kek);
    let encrypted_uid = encrypt_data(&uid.to_string(), &server_kek);

    let _ = add_dek_to_db(
        encrypted_email_kek,
        encrypted_uid,
        encrypted_dek,
        State(state.clone()),
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
