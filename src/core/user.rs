use std::env;

use crate::{
    errors::{Error, Result},
    models::{dek_model::Dek, user_model::UserResponse},
    traits::{decryption::Decrypt, encryption::Encrypt},
    utils::{
        encryption_utils::{create_dek, decrypt_data, encrypt_data},
        hashing_utils::salt_and_hash_password,
        user_utils::get_user_dek,
    },
};
use bson::{doc, oid::ObjectId, uuid, DateTime};
use futures::StreamExt;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

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
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl User {
    pub fn new_user(name: &str, email: &str, role: &str, password: &str) -> User {
        User {
            _id: ObjectId::new(),
            uid: uuid::Uuid::new().to_string(),
            name: name.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            password: password.to_string(),
            email_verified: false,
            is_active: true,
            created_at: Some(DateTime::now()),
            updated_at: Some(DateTime::now()),
        }
    }

    pub async fn encrypt_and_add(&self, mongo_client: &Client) -> Result<String> {
        let db = mongo_client.database("test");
        let new_dek = create_dek();
        let mut user = self.clone();
        user.password = salt_and_hash_password(user.password.as_str());
        let collection: Collection<User> = db.collection("users");
        match collection.insert_one(user.encrypt(&new_dek), None).await {
            Ok(_) => return Ok(new_dek),
            Err(_) => {
                return Err(Error::ServerError {
                    message: "Failed to Insert User".to_string(),
                });
            }
        }
    }

    pub async fn get_user_from_email(mongo_client: &Client, email: &str) -> Result<UserResponse> {
        // check if the payload is empty
        match email.is_empty() {
            true => Err(Error::InvalidPayload {
                message: "Invalid payload".to_string(),
            }),
            false => {
                let user_collection: Collection<User> =
                    mongo_client.database("test").collection("users");
                let dek_data = match get_user_dek(&mongo_client, email).await {
                    Ok(dek) => dek,
                    Err(e) => {
                        return Err(e);
                    }
                };

                println!("Dek Data {:?}", dek_data);

                match user_collection
                    .find_one(
                        doc! {
                            "uid": encrypt_data(&dek_data.uid, &dek_data.dek),
                        },
                        None,
                    )
                    .await
                {
                    Ok(Some(mut user)) => {
                        let decrypted_user = user.decrypt(&dek_data.dek);
                        return Ok(UserResponse {
                            name: decrypted_user.name,
                            email: decrypted_user.email,
                            role: decrypted_user.role,
                            created_at: decrypted_user.created_at,
                            updated_at: decrypted_user.updated_at,
                            email_verified: decrypted_user.email_verified,
                            is_active: decrypted_user.is_active,
                            uid: decrypted_user.uid,
                        });
                    }
                    Ok(None) => Err(Error::UserNotFound {
                        message: "User not found".to_string(),
                    }),
                    Err(_) => Err(Error::ServerError {
                        message: "Failed to get User".to_string(),
                    }),
                }
            }
        }
    }

    pub async fn get_user_from_uid(mongo_client: &Client, uid: &str) -> Result<UserResponse> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match get_user_dek(&mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        match collection
            .find_one(
                doc! {
                    "uid": encrypt_data(&dek_data.uid, &dek_data.dek),
                },
                None,
            )
            .await
        {
            Ok(Some(mut user)) => {
                let decrypted_user = user.decrypt(&dek_data.dek);
                return Ok(UserResponse {
                    name: decrypted_user.name,
                    email: decrypted_user.email,
                    role: decrypted_user.role,
                    created_at: decrypted_user.created_at,
                    updated_at: decrypted_user.updated_at,
                    email_verified: decrypted_user.email_verified,
                    is_active: decrypted_user.is_active,
                    uid: decrypted_user.uid,
                });
            }
            Ok(None) => Err(Error::UserNotFound {
                message: "User not found".to_string(),
            }),
            Err(_) => Err(Error::ServerError {
                message: "Failed to get User".to_string(),
            }),
        }
    }

    pub async fn get_all_users(mongo_client: &Client) -> Result<Vec<UserResponse>> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");
        let collection_dek: Collection<Dek> = db.collection("deks");

        // let mut cursor = collection.find(None, None).await.unwrap();
        let mut cursor_dek = collection_dek.find(None, None).await.unwrap();

        let mut users = Vec::new();
        let kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // iterate over the users and decrypt the data
        while let Some(dek) = cursor_dek.next().await {
            println!("Kek {:?}", kek);
            let dek_data: Dek = match dek {
                Ok(mut data) => data.decrypt(&kek),
                Err(_) => {
                    return Err(Error::ServerError {
                        message: "Failed to get DEK".to_string(),
                    });
                }
            };

            let encrypted_email_dek = encrypt_data(&dek_data.email, &dek_data.dek);

            // find the user in the users collection using the encrypted email to iterate over the users
            let cursor_user = collection
                .find_one(
                    Some(doc! {
                        "email": encrypted_email_dek,
                    }),
                    None,
                )
                .await
                .unwrap();

            match cursor_user {
                Some(mut user) => {
                    let user_data = user.decrypt(&dek_data.dek);

                    users.push(UserResponse {
                        name: user_data.name,
                        email: user_data.email,
                        role: user_data.role,
                        created_at: user_data.created_at,
                        updated_at: user_data.updated_at,
                        email_verified: user_data.email_verified,
                        is_active: user_data.is_active,
                        uid: user_data.uid,
                    });
                }
                None => {
                    return Err(Error::UserNotFound {
                        message: "No user found".to_string(),
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
    ) -> Result<String> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");

        let dek_data = match get_user_dek(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": encrypt_data(&dek_data.uid, &dek_data.dek),
                },
                doc! {
                    "$set": {
                        "role": encrypt_data(&role, &dek_data.dek),
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

    pub async fn toggle_user_activation(
        mongo_client: &Client,
        email: &str,
        is_active: &bool,
    ) -> Result<bool> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");
        let dek_data = match get_user_dek(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        // find the user in the users collection using the uid
        match collection
            .update_one(
                doc! {
                    "uid": encrypt_data(&dek_data.uid, &dek_data.dek),
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

    pub async fn delete(mongo_client: &Client, email: &str) -> Result<()> {
        let db = mongo_client.database("test");
        let collection: Collection<User> = db.collection("users");
        let collection_dek: Collection<Dek> = db.collection("deks");

        let dek_data = match get_user_dek(&mongo_client, email).await {
            Ok(dek) => dek,
            Err(e) => {
                return Err(e);
            }
        };

        match collection
            .delete_one(
                doc! {
                    "uid": encrypt_data(&dek_data.uid, &dek_data.dek),
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
                            "uid": encrypt_data(&dek_data.uid, &kek),
                        },
                        None,
                    )
                    .await
                {
                    Ok(cursor_dek) => {
                        if cursor_dek.deleted_count == 0 {
                            // send back a 404 to
                            return Err(Error::UserNotFound {
                                message: "DEK not found".to_string(),
                            });
                        }
                        Ok(())
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
