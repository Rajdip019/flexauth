use std::env;

use crate::{
    errors::{Error, Result},
    models::{password_model::ForgetPasswordRequest, user_model::{EmailVerificationRequest, UserResponse}},
    traits::{decryption::Decrypt, encryption::Encrypt},
    utils::{
        email_utils::Email, encryption_utils::Encryption, password_utils::Password
    },
};
use bson::{doc, oid::ObjectId, uuid, DateTime};
use futures::StreamExt;
use mongodb::{options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

use super::{dek::Dek, session::Session};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub _id: ObjectId,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub failed_login_attempts: i32,
    pub blocked_until: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl User {
    pub fn new(name: &str, email: &str, role: &str, password: &str) -> Self {
        Self {
            _id: ObjectId::new(),
            uid: uuid::Uuid::new().to_string(),
            name: name.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            password: password.to_string(),
            email_verified: false,
            is_active: true,
            failed_login_attempts: 0,
            blocked_until: None,
            created_at: Some(DateTime::now()),
            updated_at: Some(DateTime::now()),
        }
    }

    pub async fn encrypt_and_add(&self, mongo_client: &Client, dek: &str) -> Result<Self> {
        let db = mongo_client.database("auth");
        let mut user = self.clone();
        user.password = Password::salt_and_hash(user.password.as_str());
        let collection: Collection<User> = db.collection("users");
        match collection.insert_one(user.encrypt(&dek), None).await {
            Ok(_) => return Ok(user),
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to Insert User".to_string(),
                });
            }
        }
    }

    pub async fn get_from_email(mongo_client: &Client, email: &str) -> Result<User> {
                let user_collection: Collection<User> =
                    mongo_client.database("auth").collection("users");
                let dek_data = match Dek::get(&mongo_client, email).await {
                    Ok(dek) => dek,
                    Err(e) => {
                        return Err(e);
                    }
                };

                println!("Dek Data {:?}", dek_data);

                match user_collection
                    .find_one(
                        doc! {
                            "uid": dek_data.uid,
                        },
                        None,
                    )
                    .await
                {
                    Ok(Some(user)) => {
                        let decrypted_user = user.decrypt(&dek_data.dek);
                        return Ok(decrypted_user);
                    }
                    Ok(None) => Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    }),
                    Err(_) => Err(Error::ServerError {
                        message: "Failed to get User".to_string(),
                    }),
                }
    }

    pub async fn get_from_uid(mongo_client: &Client, uid: &str) -> Result<User> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match Dek::get(&mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };
        match collection
            .find_one(
                doc! {
                    "uid": &uid,
                },
                None,
            )
            .await
        {
            Ok(Some(user)) => {
                let decrypted_user = user.decrypt(&dek_data.dek);
                return Ok(decrypted_user);
            }
            Ok(None) => Err(Error::UserNotFound {
                message: "User not found".to_string(),
            }),
            Err(_) => Err(Error::ServerError {
                message: "Failed to get User".to_string(),
            }),
        }
    }

    pub async fn get_all(mongo_client: &Client) -> Result<Vec<UserResponse>> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");

        // get all the users
        let mut cursor_user = collection.find(None, None).await.unwrap();

        let mut users = Vec::new();

        // iterate over the users and decrypt the data
        while let Some(user) = cursor_user.next().await {
            let user_data = match user {
                Ok(data) => data,
                Err(_) => {
                    return Err(Error::ServerError {
                        message: "Failed to get User".to_string(),
                    });
                }
            };

            let dek_data = match Dek::get(&mongo_client, &user_data.uid).await {
                Ok(dek) => dek,
                Err(e) => {
                    return Err(e);
                }
            };

            let decrypted_user = user_data.decrypt(&dek_data.dek);

            users.push(UserResponse {
                name: decrypted_user.name,
                email: decrypted_user.email,
                role: decrypted_user.role,
                created_at: decrypted_user.created_at,
                updated_at: decrypted_user.updated_at,
                email_verified: decrypted_user.email_verified,
                blocked_until: decrypted_user.blocked_until,
                is_active: decrypted_user.is_active,
                uid: decrypted_user.uid,
            });
        }

        // sort the users by created_at
        users.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        Ok(users)
    }

    pub async fn get_recent(
        mongo_client: &Client,
        limit: i64,
    ) -> Result<Vec<UserResponse>> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");

        // get recent users from the users collection till the limit provided sort by created_at

        let find_options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .build();
        
        let mut cursor = collection
            .find(None, find_options)
            .await
            .unwrap();
        

        let mut users = Vec::new();

        // iterate over the users and decrypt the data
        while let Some(user) = cursor.next().await {
            let user_data = match user {
                Ok(data) => data,
                Err(_) => {
                    return Err(Error::ServerError {
                        message: "Failed to get User".to_string(),
                    });
                }
            };

            let dek_data = match Dek::get(&mongo_client, &user_data.uid).await {
                Ok(dek) => dek,
                Err(e) => {
                    return Err(e);
                }
            };

            let decrypted_user = user_data.decrypt(&dek_data.dek);

            users.push(UserResponse {
                name: decrypted_user.name,
                email: decrypted_user.email,
                role: decrypted_user.role,
                blocked_until: decrypted_user.blocked_until,
                created_at: decrypted_user.created_at,
                updated_at: decrypted_user.updated_at,
                email_verified: decrypted_user.email_verified,
                is_active: decrypted_user.is_active,
                uid: decrypted_user.uid,
            });
        }

        // sort the users by created_at
        users.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // get the recent users
        let recent_users = users.iter().rev().take(limit as usize).cloned().collect();

        Ok(recent_users)
    }

    pub async fn update_role(
        mongo_client: &Client,
        email: &str,
        role: &str,
    ) -> Result<String> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");

        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": &dek_data.uid,
                },
                doc! {
                    "$set": {
                        "role": Encryption::encrypt_data(&role, &dek_data.dek),
                        "updated_at": DateTime::now(),
                    }
                },
                None,
            )
            .await
        {
            Ok(cursor) => {
                let modified_count = cursor.modified_count;

                // Return Error if User is not there
                if modified_count == 0 {
                    // send back a 404 to
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }
                return Ok(role.to_string());
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to update User".to_string(),
                })
            }
        }
    }

    pub async fn toggle_account_activation(
        mongo_client: &Client,
        email: &str,
        is_active: &bool,
    ) -> Result<bool> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": &dek_data.uid,
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
        {
            Ok(cursor) => {
                let modified_count = cursor.modified_count;

                // Return Error if User is not there
                if modified_count == 0 {
                    // send back a 404 to
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }
                return Ok(is_active.to_owned());
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to update User".to_string(),
                })
            }
        }
    }

    pub async fn increase_failed_login_attempt(
        mongo_client: &Client,
        email: &str,
    ) -> Result<i32> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": &dek_data.uid,
                },
                doc! {
                    "$inc": {
                        "failed_login_attempts": 1
                    },
                    "$set": {
                        "updated_at": DateTime::now(),
                    }
                },
                None,
            )
            .await
        {
            Ok(cursor) => {
                let modified_count = cursor.modified_count;

                // Return Error if User is not there
                if modified_count == 0 {
                    // send back a 404 to
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }

                // check if the failed login attempts is greater than 5 then add_block_user_until 180 seconds and if it is 10 then for 10 minutes if its 15 then 1hr
                let user = match User::get_from_email(&mongo_client, email).await {
                    Ok(user) => user,
                    Err(e) => {
                        return Err(e);
                    }
                };

                if user.failed_login_attempts == 5 {
                    let blocked_until = DateTime::now().timestamp_millis() + 180000;

                    // send a email to the user to notify multiple login attempts detected
                    Email::new(
                        &user.name,
                        &user.email,
                        &"Multiple login Attempts detected",
                        &("We have detected an multiple unauthorized login attempt associated with your account. For your security, we have taken action to protect your account.

                            If you attempted to log in, please disregard this message. However, if you did not attempt to log in, we recommend taking the following steps:

                            Immediately change your password to a strong, unique one.
                            Review your account activity for any suspicious activity.
                            If you have any concerns or questions, please don't hesitate to contact our support team.

                            Stay safe and secure,
                            FlexAuth Team"),
                    ).send().await;

                    match collection
                        .update_one(
                            doc! {
                                "uid": &dek_data.uid,
                            },
                            doc! {
                                "$set": {
                                    "blocked_until": DateTime::from_millis(blocked_until),
                                    "updated_at": DateTime::now(),
                                }
                            },
                            None,
                        )
                        .await
                    {
                        Ok(_) => {
                            return Ok(5);
                        }
                        Err(_) => {
                            return Err(Error::ServerError {
                                message: "Failed to update User".to_string(),
                            });
                        }
                    }
                } else if user.failed_login_attempts == 10 {
                    let blocked_until = DateTime::now().timestamp_millis() + 600000;

                    // send a email to the user to notify multiple login attempts detected
                    Email::new(
                        &user.name,
                        &user.email,
                        &"Multiple login Attempts detected",
                        &("We have detected an multiple unauthorized login attempt associated with your account. For your security, we have taken action to protect your account.

                            If you attempted to log in, please disregard this message. However, if you did not attempt to log in, we recommend taking the following steps:

                            Immediately change your password to a strong, unique one.
                            Review your account activity for any suspicious activity.
                            If you have any concerns or questions, please don't hesitate to contact our support team.

                            Stay safe and secure,
                            FlexAuth Team"),
                    ).send().await;

                    match collection
                        .update_one(
                            doc! {
                                "uid": &dek_data.uid,
                            },
                            doc! {
                                "$set": {
                                    "blocked_until": DateTime::from_millis(blocked_until),
                                    "updated_at": DateTime::now(),
                                }
                            },
                            None,
                        )
                        .await
                    {
                        Ok(_) => {
                            return Ok(10);
                        }
                        Err(_) => {
                            return Err(Error::ServerError {
                                message: "Failed to update User".to_string(),
                            });
                        }
                    }
                } else if user.failed_login_attempts == 15 {
                    let blocked_until = DateTime::now().timestamp_millis() + 3600000;

                    // send a email to the user to notify multiple login attempts detected
                    Email::new(
                        &user.name,
                        &user.email,
                        &"Multiple login Attempts detected",
                        &("We have detected an multiple unauthorized login attempt associated with your account. For your security, we have taken action to protect your account.

                            If you attempted to log in, please disregard this message. However, if you did not attempt to log in, we recommend taking the following steps:

                            Immediately change your password to a strong, unique one.
                            Review your account activity for any suspicious activity.
                            If you have any concerns or questions, please don't hesitate to contact our support team.

                            Stay safe and secure,
                            FlexAuth Team"),
                    ).send().await;

                    match collection
                        .update_one(
                            doc! {
                                "uid": &dek_data.uid,
                            },
                            doc! {
                                "$set": {
                                    "blocked_until": DateTime::from_millis(blocked_until),
                                    "updated_at": DateTime::now(),
                                }
                            },
                            None,
                        )
                        .await
                    {
                        Ok(_) => {
                            return Ok(15);
                        }
                        Err(_) => {
                            return Err(Error::ServerError {
                                message: "Failed to update User".to_string(),
                            });
                        }
                    }
                } else {
                    return Ok(user.failed_login_attempts);
                }
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to update User".to_string(),
                })
            }
        }
    }

    pub async fn reset_failed_login_attempt(
        mongo_client: &Client,
        email: &str,
    ) -> Result<String> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": &dek_data.uid,
                },
                doc! {
                    "$set": {
                        "failed_login_attempts": 0,
                        "updated_at": DateTime::now(),
                    }
                },
                None,
            )
            .await
        {
            Ok(cursor) => {
                let modified_count = cursor.modified_count;

                // Return Error if User is not there
                if modified_count == 0 {
                    // send back a 404 to
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }
                return Ok("Failed login attempts reset".to_string());
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to update User".to_string(),
                })
            }
        }
    }

    pub async fn change_password(mongo_client: &Client, email: &str, old_password: &str, new_password: &str) -> Result<String> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };
        // get user
        let user = match collection
            .find_one(
                doc! { "email": Encryption::encrypt_data(&email, &dek_data.dek) },
                None,
            )
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(Error::UserNotFound {
                    message: "User not found".to_string(),
                });
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to get User".to_string(),
                });
            }
        };

        let decrypted_user = user.decrypt(&dek_data.dek);
        
        if !Password::verify_hash(&old_password, &decrypted_user.password) {
            return Err(Error::InvalidPassword {
                message: "New password cannot be the same as the old password".to_string(),
            });
        }
        // hash and salt the new password
        let hashed_and_salted_pass = Password::salt_and_hash(&new_password);
        // encrypt the new password
        let encrypted_password = Encryption::encrypt_data(&hashed_and_salted_pass, &dek_data.dek);

        // update the user with the new password
        match collection
            .find_one_and_update(
                doc! { "email": Encryption::encrypt_data(&email, &dek_data.dek) },
                doc! {
                    "$set": {
                        "password": encrypted_password,
                        "updated_at": DateTime::now(),
                    }
                },
                None,
            )
            .await
        {
            Ok(_) => { return Ok("Password updated successfully".to_string())}
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to update User".to_string(),
                });
            }
        }
    }

    pub async fn forget_password_request(mongo_client: &Client, email: &str) -> Result<String> {
        // check if the user exists
        let db = mongo_client.database("auth");
        let dek_data = match Dek::get(&mongo_client, &email).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let user = match User::get_from_email(&mongo_client, &email).await {
            Ok(user) => user,
            Err(e) => return Err(e)
        };

        // get a time 10 minutes from now
        let ten_minutes_from_now_millis = DateTime::now().timestamp_millis() + 600000;
        let ten_minutes_from_now = DateTime::from_millis(ten_minutes_from_now_millis);
        // create a new doc in forget_password_requests collection
        let new_doc = ForgetPasswordRequest {
            _id: ObjectId::new(),
            id: uuid::Uuid::new().to_string(),
            email: Encryption::encrypt_data(&email, &dek_data.dek),
            is_used: false,
            valid_till: ten_minutes_from_now,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        db.collection("forget_password_requests")
            .insert_one(new_doc.clone(), None)
            .await
            .unwrap();

        // send a email to the user with the link having id of the new doc
        Email::new(
            &user.name,
            &user.email,
            &"Reset Password",
            &format!("Please click on the link to reset your password: http://localhost:8080/forget-reset/{}", new_doc.id),
        ).send().await;

        Ok("Forget password request sent to email successfully".to_string())
    }

    pub async fn forget_password_reset(
        mongo_client: &Client,
        req_id: &str,
        email: &str,
        new_password: &str
    ) -> Result<String> {
        let db = mongo_client.database("auth");
        let user_collection: Collection<User> = db.collection("users");
        let forget_password_requests_collection: Collection<ForgetPasswordRequest> =
            db.collection("forget_password_requests");

        // find the dek with the email
        let dek_data = match Dek::get(&mongo_client, &email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // check if forget password request exists
        let forget_password_request = forget_password_requests_collection
            .find_one(doc! { "id": &req_id }, None)
            .await
            .unwrap()
            .unwrap();

        println!("Forget Password Request {:?}", forget_password_request);

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
        let user = match User::get_from_email(&mongo_client, &email).await {
            Ok(user) => user,
            Err(e) => {
                return Err(e);
            }
        };

        // hash and salt the new password
        let hashed_and_salted_pass = Password::salt_and_hash(&new_password);
        // encrypt the new password
        let encrypted_password = Encryption::encrypt_data(&hashed_and_salted_pass, &dek_data.dek);

        // update the user with the new password
        user_collection
            .find_one_and_update(
                doc! { "email": Encryption::encrypt_data(&email, &dek_data.dek) },
                doc! {
                    "$set": {
                        "password": encrypted_password,
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
                doc! { "id": &req_id },
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
        Email::new( 
            &user.name,
            &email,
            &"Password Updated", 
            &"Your password has been updated successfully. If it was not you please take action as soon as possible",
        ).send().await;

        Ok("Password updated successfully".to_string())

    }

    pub async fn verify_email_request(mongo_client: &Client, email: &str) -> Result<EmailVerificationRequest> {
        // make a new request in the email_verification_requests collection
        let db = mongo_client.database("auth");
        let collection: Collection<EmailVerificationRequest> = db.collection("email_verification_requests");

        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // get a time 24h from now
        let twenty_four_hours_from_now_millis = DateTime::now().timestamp_millis() + 86400000;
        let twenty_four_hours_from_now = DateTime::from_millis(twenty_four_hours_from_now_millis);

        let new_doc = EmailVerificationRequest {
            _id: ObjectId::new(),
            uid: dek_data.uid,
            req_id: uuid::Uuid::new().to_string(),
            email: Encryption::encrypt_data(&email, &dek_data.dek),
            // expires in 24 hours
            expires_at: twenty_four_hours_from_now,
            created_at: Some(DateTime::now()),
            updated_at: Some(DateTime::now()),
        };

        collection.insert_one(&new_doc, None).await.unwrap();

        // send a email to the user with the link having id of the new doc
        Email::new(
            &"FlexAuth Team",
            &email,
            &"Verify Email",
            &format!("Please click on the link to verify your email: http://localhost:8080/verify-email/{}", new_doc.req_id),
        ).send().await;

        Ok(new_doc)
    }

    pub async fn verify_email(mongo_client: &Client, req_id: &str) -> Result<String> {
        // check if the email_verification_request exists
        let db = mongo_client.database("auth");
        let collection: Collection<EmailVerificationRequest> = db.collection("email_verification_requests");

        let email_verification_request = match collection
            .find_one(doc! { "req_id": req_id }, None)
            .await
            .unwrap() {
            Some(data) => data,
            None => {
                return Err(Error::UserNotFound {
                    message: "Email verification request not found. Please request a new link.".to_string(),
                });
            }
        };

        // check if the request exists
        if email_verification_request.email.is_empty() {
            return Err(Error::UserNotFound {
                message: "Email verification request not found. Please request a new link.".to_string(),
            });
        }

        if email_verification_request.expires_at.timestamp_millis() < DateTime::now().timestamp_millis() {
            return Err(Error::EmailVerificationLinkExpired {
                message: "The link has expired. Please request a new link.".to_string(),
            });
        }

        // update the user with verified email
        let user_collection: Collection<User> = db.collection("users");

        user_collection
            .find_one_and_update(
                doc! { "uid": &email_verification_request.uid },
                doc! {
                    "$set": {
                        "email_verified": true,
                        "updated_at": DateTime::now(),
                    }
                },
                None,
            )
            .await
            .unwrap();

        // delete the email_verification_request
        collection
            .delete_one(doc! { "req_id": req_id }, None)
            .await
            .unwrap();

        let dek_data = match Dek::get(&mongo_client, &email_verification_request.uid).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        let decrypted_email = Encryption::decrypt_data(&email_verification_request.email, &dek_data.dek);

        // send a email to the user that the email has been verified
        Email::new(
            &"FlexAuth Team",
            &decrypted_email,
            &"Email Verified",
            &"Your email has been verified successfully. If it was not you please take action as soon as possible",
        ).send().await;
        Ok(req_id.to_string())
    }

    pub async fn delete(mongo_client: &Client, email: &str) -> Result<String> {
        let db = mongo_client.database("auth");
        let collection: Collection<User> = db.collection("users");
        let collection_dek: Collection<Dek> = db.collection("deks");

        let dek_data = match Dek::get(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        match collection
            .delete_one(
                doc! {
                    "uid": &dek_data.uid,
                },
                None,
            )
            .await
        {
            Ok(cursor) => {
                if cursor.deleted_count == 0 {
                    // send back a 404 to
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }

                let kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

                // delete the dek from the deks collection
                match collection_dek
                    .delete_one(
                        doc! {
                            "uid": Encryption::encrypt_data(&dek_data.uid, &kek),
                        },
                        None,
                    )
                    .await
                {
                    Ok(cursor_dek) => {
                        // delete all sessions associated with the user
                        let session_collection: Collection<Session> = db.collection("sessions");
                        session_collection
                            .delete_many(
                                doc! {
                                    "uid": Encryption::encrypt_data(&dek_data.uid, &kek),
                                },
                                None,
                            )
                            .await
                            .unwrap();
                        
                        if cursor_dek.deleted_count == 0 {
                            // send back a 404 to
                            return Err(Error::UserNotFound {
                                message: "DEK not found".to_string(),
                            });
                        }
                        Ok(dek_data.uid)
                    }
                    Err(_) => {
                        return Err(Error::ServerError {
                            message: "Failed to delete DEK".to_string(),
                        });
                    }
                }
            }
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to delete User".to_string(),
                })
            }
        }
    }
}
