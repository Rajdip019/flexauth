use crate::{
    errors::{Error, Result},
    models::{dek_model::Dek, forget_password_req_model::{ForgetPasswordRequest, ForgetPasswordRequestPayload, ForgetPasswordResetPayload, NewForgetPasswordRequest}, password_model::ResetPasswordPayload, user_model::User},
    utils::{
        email::{send_email, Email},
        encryption_utils::{decrypt_data, encrypt_data},
        hashing_utils::{salt_and_hash_password, verify_password},
    },
    AppState,
};
use axum::{extract::{Path, State}, Json};
use axum_macros::debug_handler;
use bson::{doc, uuid, DateTime};
use mongodb::Collection;
// use futures::stream::StreamExt;
use serde_json::{json, Value};
use std::env;

#[debug_handler]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    payload: Json<ResetPasswordPayload>,
) -> Result<Json<Value>> {
    let db = state.mongo_client.database("test");
    let dek_collection: Collection<Dek> = db.collection("deks");
    let user_collection: Collection<User> = db.collection("users");

    // check if the

    // encrypt the email with kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_email = encrypt_data(&payload.email, &server_kek);

    // find the dek with the email
    let dek_data = dek_collection
        .find_one(doc! { "email": encrypted_email }, None)
        .await
        .unwrap()
        .unwrap();

    // return if the dek is not found
    if dek_data.dek.is_empty() {
        return Err(Error::UserNotFound {
            message: "User not found. Please check the email and try again.".to_string(),
        });
    }
    // decrypt the dek with the server kek
    let dek = decrypt_data(&dek_data.dek, &server_kek);

    let dek_encrypted_email = encrypt_data(&payload.email, &dek);

    // check if the user exists
    let user = user_collection
        .find_one(doc! { "email": dek_encrypted_email }, None)
        .await
        .unwrap()
        .unwrap();

    // decrypt the password and salt using the dek
    let password_hashed = decrypt_data(user.password.split('.').collect::<Vec<&str>>()[0], &dek);
    let salt = decrypt_data(user.password.split('.').collect::<Vec<&str>>()[1], &dek);

    // verify the password
    let is_valid = verify_password(&payload.old_password, &salt, &password_hashed);
    println!("is_valid: {}", is_valid);

    if !is_valid {
        return Err(Error::InvalidPassword {
            message: "Invalid password. Please check the password and try again.".to_string(),
        });
    }

    // hash and salt the new password
    let hashed_and_salted_pass = salt_and_hash_password(&payload.new_password);
    // encrypt the new password
    let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &dek);
    let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &dek);

    // update the user with the new password
    user_collection
        .update_one(
            doc! { "email": encrypt_data(&payload.email, &dek) },
            doc! {
                "$set": {
                    "password": format!("{}.{}", encrypted_password, encrypted_salt),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    // send a email to the user that the password has been updated
    send_email(Email {
        name: user.name,
        email: payload.email.clone(),
        subject: "Password Updated".to_string(),
        body: "Your password has been updated successfully. If it was not you please take action as soon as possible".to_string(),
    }).await;

    return Ok(Json(json!({
        "message": "Password updated successfully. Please login with the new password."
    })));
}

#[debug_handler]
pub async fn forgot_password_request_handler(
    State(state): State<AppState>,
    payload: Json<ForgetPasswordRequestPayload>,
) -> Result<Json<Value>> {
    // check if payload.email exists
    if payload.email.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Email is required.".to_string(),
        });
    }
    // check if the user exists
    let db = state.mongo_client.database("test");
    let user_collection: Collection<User> = db.collection("users");
    let dek_collection: Collection<Dek> = db.collection("deks");

    let dek_data = dek_collection
        .find_one(doc! { "email": encrypt_data(&payload.email, &env::var("SERVER_KEK").unwrap()) }, None)
        .await
        .unwrap()
        .unwrap();

    // return if the dek is not found
    if dek_data.dek.is_empty() {
        return Err(Error::UserNotFound {
            message: "User not found. Please check the email and try again.".to_string(),
        });
    }

    // decrypt the dek with the server kek
    let dek = decrypt_data(&dek_data.dek, &env::var("SERVER_KEK").unwrap());

    let user = user_collection
        .count_documents(doc! { "email": encrypt_data(&payload.email, &dek) }, None)
        .await
        .unwrap();

    if user == 0 {
        return Err(Error::UserNotFound {
            message: "User not found. Please check the email and try again.".to_string(),
        });
    }

    // get a time 10 minutes from now
    let ten_minutes_from_now_millis = DateTime::now().timestamp_millis() + 600000;
    let ten_minutes_from_now = DateTime::from_millis(ten_minutes_from_now_millis);
    // create a new doc in forget_password_requests collection
    let new_doc = NewForgetPasswordRequest {
        id: uuid::Uuid::new().to_string(),
        email: encrypt_data(&payload.email, &dek),
        is_used: false,
        valid_till: ten_minutes_from_now,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    db.collection("forget_password_requests").insert_one(new_doc.clone(), None).await.unwrap();

    // send a email to the user with the link having id of the new doc
    send_email(Email {
        name: "User".to_string(),
        email: payload.email.clone(),
        subject: "Reset Password".to_string(),
        body: format!("Please click on the link to reset your password: http://localhost:8080/forget-password-reset/{}", new_doc.id),
    }).await;

    return Ok(Json(json!({
        "message": "A reset password link has been sent to your email. Please check your email."
    })));
}

#[debug_handler]
pub async fn forget_reset_password_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    payload: Json<ForgetPasswordResetPayload>,
) -> Result<Json<Value>> {
    // check if payload is valid
    if payload.email.is_empty() | payload.password.is_empty() {
        return Err(Error::InvalidPayload {
            message: "Email is required.".to_string(),
        });
    }

    let db = state.mongo_client.database("test");
    let dek_collection: Collection<Dek> = db.collection("deks");
    let user_collection: Collection<User> = db.collection("users");
    let forget_password_requests_collection: Collection<ForgetPasswordRequest> = db.collection("forget_password_requests");

    println!("id: {}", id);

    // encrypt the email with kek
    let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
    let encrypted_email = encrypt_data(&payload.email, &server_kek);

    // find the dek with the email
    let dek_data = dek_collection
        .find_one(doc! { "email": encrypted_email }, None)
        .await
        .unwrap()
        .unwrap();

    // return if the dek is not found
    if dek_data.dek.is_empty() {
        return Err(Error::UserNotFound {
            message: "User not found. Please check the email and try again.".to_string(),
        });
    }
    // decrypt the dek with the server kek
    let dek = decrypt_data(&dek_data.dek, &server_kek);

    let dek_encrypted_email = encrypt_data(&payload.email, &dek);

    println!("dek_encrypted_email: {}", dek_encrypted_email);

    // check if forget password request exists
    let forget_password_request = forget_password_requests_collection
        .find_one(doc! { "id": &id }, None)
        .await
        .unwrap()
        .unwrap();

    if forget_password_request.is_used {
        return Err(Error::ResetPasswordLinkExpired {
            message: "The link has already been used. Please request a new link.".to_string(),
        });
    }

    //  check if forget password request exists
    if forget_password_request.email.is_empty() {
        return Err(Error::UserNotFound {
            message: "Forget password request not found. Please request a new link.".to_string(),
        });
    }

    // check if the request is valid
    if forget_password_request.valid_till.timestamp_millis() < DateTime::now().timestamp_millis() {
        return Err(Error::ResetPasswordLinkExpired {
            message: "The link has expired. Please request a new link.".to_string(),
        });
    }

    // check if the user exists
    let user = user_collection
        .find_one(doc! { "email": dek_encrypted_email }, None)
        .await
        .unwrap()
        .unwrap();

    //  check if user exists
    if user.email.is_empty() {
        return Err(Error::UserNotFound {
            message: "User not found. Please check the email and try again.".to_string(),
        });
    }

    println!("user: {:?}", user);

    // hash and salt the new password
    let hashed_and_salted_pass = salt_and_hash_password(&payload.password);
    // encrypt the new password
    let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &dek);
    let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &dek);

    // update the user with the new password
    user_collection
        .find_one_and_update(
            doc! { "email": encrypt_data(&payload.email, &dek) },
            doc! {
                "$set": {
                    "password": format!("{}.{}", encrypted_password, encrypted_salt),
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    // update the forget password request as used
    forget_password_requests_collection
        .find_one_and_update(
            doc! { "id": id },
            doc! {
                "$set": {
                    "is_used": true,
                    "updated_at": DateTime::now(),
                }
            },
            None,
        )
        .await
        .unwrap();

    // send a email to the user that the password has been updated
    send_email(Email {
        name: user.name,
        email: payload.email.clone(),
        subject: "Password Updated".to_string(),
        body: "Your password has been updated successfully. If it was not you please take action as soon as possible".to_string(),
    }).await;

    return Ok(Json(json!({
        "message": "Password updated successfully. Please login with the new password."
    })));
}