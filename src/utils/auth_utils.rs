use axum::Json;
use bson::doc;
use mongodb::{Client, Collection};
use serde_json::{json, Value};

use crate::{
    core::{dek::Dek, user::User},
    errors::{Error, Result},
    models::auth_model::{SignInPayload, SignUpPayload},
    traits::decryption::Decrypt,
    utils::{
        encryption_utils::encrypt_data, hashing_utils::verify_password_hash,
        session_utils::sign_jwt,
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

    let dek = Dek::generate(); // create a data encryption key for new user
    match User::new(
        &payload.name,
        &payload.email,
        &payload.role,
        &payload.password,
    )
    .encrypt_and_add(&mongo_client, &dek)
    .await
    {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let mut user = match collection
        .find_one(
            Some(doc! {
                "email": encrypt_data(&payload.email, &dek)
            }),
            None,
        )
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err(Error::UserNotFound {
                    message: "User not found".to_string(),
                });
            }
        },
        Err(e) => {
            return Err(Error::ServerError {
                message: e.to_string(),
            })
        }
    };

    let decrypted_user = user.decrypt(&dek);

    // add the dek to the deks collection
    match Dek::new(&decrypted_user.uid, &decrypted_user.email, &dek)
        .encrypt_and_add(&mongo_client)
        .await
    {
        Ok(dek_data) => dek_data,
        Err(e) => return Err(e),
    };

    println!(">> User added successfully: {:?}", decrypted_user);

    // issue a jwt token
    let token = match sign_jwt(&decrypted_user) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Error signing jwt token: {}", err);
            return Err(Error::IdTokenCreationError {
                message: err.to_string(),
            });
        }
    };

    Ok(Json(json!({
            "uid": decrypted_user.uid,
            "name": decrypted_user.name,
            "email": decrypted_user.email,
            "role": decrypted_user.role,
            "created_at": decrypted_user.created_at,
            "updated_at": decrypted_user.updated_at,
            "email_verified": decrypted_user.email_verified,
            "is_active": decrypted_user.is_active,
            "token": token,
    })))
}

pub async fn sign_in(mongo_client: &Client, payload: Json<SignInPayload>) -> Result<Json<Value>> {
    // check if the payload is empty
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }
    let db = mongo_client.database("test");

    let dek_data = match Dek::get(&mongo_client, &payload.email).await {
        Ok(dek) => dek,
        Err(e) => return Err(e),
    };

    let encrypted_uid = encrypt_data(&dek_data.uid, &dek_data.dek);
    // find the user in the users collection using the uid
    let collection: Collection<User> = db.collection("users");
    let mut user = match collection
        .find_one(
            Some(doc! {
                "uid": encrypted_uid
            }),
            None,
        )
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err(Error::UserNotFound {
                    message: "User not found".to_string(),
                });
            }
        },
        Err(e) => {
            return Err(Error::ServerError {
                message: e.to_string(),
            })
        }
    };

    let decrypted_user = user.decrypt(&dek_data.dek);

    // verify the password
    if verify_password_hash(&payload.password, &decrypted_user.password) {
        // issue a jwt token
        let token = match sign_jwt(&decrypted_user) {
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
