use axum::Json;
use bson::doc;
use mongodb::{Client, Collection};
use serde_json::{json, Value};

use crate::{
    core::{dek::Dek, session::Session, user::User},
    errors::{Error, Result},
    models::auth_model::{SignInPayload, SignUpPayload},
    utils::{encryption_utils::Encryption, hashing_utils::verify_password_hash},
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
    let user = match User::new(
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

    // add the dek to the deks collection
    let dek_data = match Dek::new(&user.uid, &user.email, &dek)
        .encrypt_and_add(&mongo_client)
        .await
    {
        Ok(dek_data) => dek_data,
        Err(e) => return Err(e),
    };

    let session = match Session::new(&user, "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36").encrypt_add(&mongo_client, &dek).await {
        Ok(session) => session,
        Err(e) => return Err(e),
    };

    Ok(Json(json!({
            "uid": user.uid,
            "name": user.name,
            "email": user.email,
            "role": user.role,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
            "email_verified": user.email_verified,
            "is_active": user.is_active,
            "session": {
                "session_id": Encryption::encrypt_data(&session.session_id, &dek_data.dek),
                "id_token" : session.id_token,
                "refresh_token" : session.refresh_token,
            },
    })))
}

pub async fn sign_in(mongo_client: &Client, payload: Json<SignInPayload>) -> Result<Json<Value>> {
    // check if the payload is empty
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Invalid payload".to_string(),
        });
    }

    let user = match User::get_from_email(&mongo_client, &payload.email).await {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let dek_data = match Dek::get(&mongo_client, &user.uid).await {
        Ok(dek_data) => dek_data,
        Err(e) => return Err(e),
    };

    // verify the password
    if verify_password_hash(&payload.password, &user.password) {
        let session = match Session::new(&user, "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36").encrypt_add(&mongo_client, &dek_data.dek).await {
            Ok(session) => session,
            Err(e) => return Err(e),
        };
        let res = Json(json!({
            "message": "Signin successful",
            "user": {
                "uid": user.uid,
                "name": user.name,
                "email": user.email,
                "role": user.role,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
                "email_verified": user.email_verified,
                "is_active": user.is_active,
                "session": {
                    "session_id": Encryption::encrypt_data(&session.session_id, &dek_data.dek),
                    "id_token" : session.id_token,
                    "refresh_token" : session.refresh_token,
                },
            },
        }));

        Ok(res)
    } else {
        Err(Error::UserNotFound {
            message: "User not found".to_string(),
        })
    }
}
