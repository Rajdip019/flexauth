use std::env;

use crate::{
    errors::{Error, Result},
    models::{
        dek_model::Dek,
        user_model::{User, UserResponse},
    },
};
use axum::Json;
use bson::{doc, uuid, DateTime};
use futures::StreamExt;
use mongodb::{results::InsertOneResult, Client, Collection};
use serde_json::{json, Value};

use crate::models::user_model::NewUser;

use super::encryption_utils::{create_dek, decrypt_data, encrypt_data, encrypt_user};

impl User {
    pub async fn add(&self, mongo_client: &Client) -> Result<InsertOneResult> {
        let db = mongo_client.database("test");
        let new_dek = create_dek();
        let encrypted_user = encrypt_user(self, &new_dek);
        let collection: Collection<User> = db.collection("users");
        match collection.insert_one(&encrypted_user, None).await {
            Ok(uid) => return Ok(uid),
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to Insert User".to_string(),
                });
            }
        }
    }

    pub fn new_user(name: &str, email: &str, role: &str, password: &str) -> NewUser {
        NewUser {
            uid: uuid::Uuid::new().to_string(),
            name: name.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            password: password.to_string(),
            email_verified: false,
            is_active: true,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn get_user_from_email(mongo_client: &Client, email: &str) -> Result<UserResponse> {
        // check if the payload is empty
        match email.is_empty() {
            true => Err(Error::InvalidPayload {
                message: "Invalid payload".to_string(),
            }),
            false => {
                let db = mongo_client.database("test");
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

                let dek_data = match cursor_dek {
                    Some(dek) => dek,
                    None => {
                        return Err(Error::KeyNotFound {
                            message: "DEK not found".to_string(),
                        });
                    }
                };

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

                let user_data = match cursor_user {
                    Some(user) => UserResponse {
                        name: user.name,
                        email: decrypt_data(&user.email, &dek_data.dek),
                        role: decrypt_data(&user.role, &dek_data.dek),
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                        email_verified: user.email_verified,
                        is_active: user.is_active,
                        uid: user.uid,
                    },
                    None => {
                        return Err(Error::UserNotFound {
                            message: "User not found".to_string(),
                        });
                    }
                };

                Ok(user_data)
            }
        }
    }

    pub async fn get_user_from_uid(mongo_client: &Client, uid: &str) -> Result<UserResponse> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");
        let collection_dek: Collection<Dek> = db.collection("deks");

        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // encrypt the email using kek
        let encrypted_uid = encrypt_data(&uid, &server_kek);

        let cursor_dek = collection_dek
            .find_one(
                Some(doc! {
                    "uid": &encrypted_uid,
                }),
                None,
            )
            .await
            .unwrap();

        let dek_data = match cursor_dek {
            Some(dek) => dek,
            None => {
                return Err(Error::KeyNotFound {
                    message: "DEK not found".to_string(),
                });
            }
        };

        let cursor_user = collection
            .find_one(
                doc! {
                    "uid": uid.clone(),
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

        let res = UserResponse {
            name: user_data.name,
            email: decrypt_data(&user_data.email, &dek_data.dek),
            role: decrypt_data(&user_data.role, &dek_data.dek),
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
            email_verified: user_data.email_verified,
            is_active: user_data.is_active,
            uid: user_data.uid,
        };

        Ok(res)
    }

    pub async fn add_user_to_db(mongo_client: &Client, user: &User) -> Result<InsertOneResult> {
        let db = mongo_client.database("test");
        let new_dek = create_dek();
        let encrypted_user = encrypt_user(user, &new_dek);
        let collection: Collection<User> = db.collection("users");
        match collection.insert_one(&encrypted_user, None).await {
            Ok(uid) => return Ok(uid),
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to Insert User".to_string(),
                });
            }
        }
    }

    pub async fn get_all_users(mongo_client: &Client) -> Result<Vec<UserResponse>> {
        let db = mongo_client.database("test");
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

            match cursor_user {
                Some(user) => {
                    let user_data = user;

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
                None => {
                    return Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    });
                }
            }
        }

        Ok(users)
    }

    pub async fn update_user_role(
        mongo_client: &Client,
        email: &str,
        role: &str,
    ) -> Result<Json<Value>> {
        let db = mongo_client.database("test");
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
        mongo_client: &Client,
        email: &str,
        is_active: &bool,
    ) -> Result<Json<Value>> {
        let db = mongo_client.database("test");
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

    pub async fn delete_user(mongo_client: &Client, email: &str) -> Result<Json<Value>> {
        let db = mongo_client.database("test");
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
}
