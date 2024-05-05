use crate::{
    errors::{Error, Result},
    models::session_model::SessionResponse,
    traits::{decryption::Decrypt, encryption::Encrypt},
    utils::{
        encryption_utils::Encryption,
        session_utils::{IDToken, RefreshToken},
    },
};
use bson::{doc, DateTime};
use futures::StreamExt;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use super::{dek::Dek, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub uid: String,
    pub email: String,
    pub id_token: String,
    pub refresh_token: String,
    pub user_agent: String,
    pub is_revoked: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Session {
    pub fn new(user: &User, user_agent: &str) -> Self {
        let id_token = match IDToken::new(user).sign() {
            Ok(token) => token,
            Err(_) => "".to_string(),
        };

        let refresh_token = match RefreshToken::new(&user.uid).sign() {
            Ok(token) => token,
            Err(_) => "".to_string(),
        };

        Self {
            uid: user.uid.to_string(),
            email: user.email.to_string(),
            id_token,
            refresh_token,
            user_agent: user_agent.to_string(),
            is_revoked: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn encrypt_add(&self, mongo_client: &Client, key: &str) -> Result<Self> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        let encrypted_session = self.encrypt(key);

        match collection_session.insert_one(encrypted_session, None).await {
            Ok(_) => Ok(self.clone()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn verify(mongo_client: &Client, id_token: &str) -> Result<(IDToken, bool)> {
        let token_data = match IDToken::verify(&id_token) {
            Ok(token_verify_result) => {
                //  check if the session is expired using the boolean
                if !token_verify_result.1 {
                    return Ok(token_verify_result);
                }
                let db = mongo_client.database("test");
                let collection_session: Collection<Session> = db.collection("sessions");

                let dek_data = match Dek::get(mongo_client, &token_verify_result.0.uid).await {
                    Ok(dek) => dek,
                    Err(e) => return Err(e),
                };

                let encrypted_id =
                    Encryption::encrypt_data(&token_verify_result.0.uid, &dek_data.dek);
                let encrypted_id_token = Encryption::encrypt_data(&id_token, &dek_data.dek);

                let session = match collection_session
                    .count_documents(
                        doc! {
                            "uid": encrypted_id,
                            "id_token": encrypted_id_token,
                            "is_revoked": false,
                        },
                        None,
                    )
                    .await
                {
                    Ok(count) => {
                        if count == 1 {
                            Ok(())
                        } else {
                            Err(Error::SessionExpired {
                                message: "Invalid token".to_string(),
                            })
                        }
                    }
                    Err(e) => Err(Error::ServerError {
                        message: e.to_string(),
                    }),
                };
                if session.is_err() {
                    return Err(Error::InvalidToken {
                        message: "Invalid token".to_string(),
                    });
                } else {
                    Ok(token_verify_result)
                }
            }
            Err(e) => return Err(e),
        };
        token_data
    }

    pub async fn refresh_session(
        mongo_client: &Client,
        id_token: &str,
        refresh_token: &str,
    ) -> Result<(String, String)> {
        // verify refresh token 
        match RefreshToken::verify(&refresh_token) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        match Self::verify(&mongo_client, &id_token).await {
            Ok(token_verify_result) => {
                if !token_verify_result.1 {
                    let db = mongo_client.database("test");
                    let collection_session: Collection<Session> = db.collection("sessions");

                    let dek_data = match Dek::get(mongo_client, &token_verify_result.0.uid).await {
                        Ok(dek) => dek,
                        Err(e) => return Err(e),
                    };

                    let encrypted_id = Encryption::encrypt_data(&token_verify_result.0.uid, &dek_data.dek);
                    let encrypted_id_token = Encryption::encrypt_data(&id_token, &dek_data.dek);
                    let encrypted_refresh_token =
                        Encryption::encrypt_data(&refresh_token, &dek_data.dek);

                    match collection_session
                        .count_documents(
                            doc! {
                                "uid": &encrypted_id,
                                "id_token": &encrypted_id_token,
                                "refresh_token": &encrypted_refresh_token,
                                "is_revoked": false,
                            },
                            None,
                        )
                        .await
                    {
                        Ok(count) => {
                            if count == 1 {
                                // generate a new id token and refresh token
                                let user = match User::get_from_uid(&mongo_client, &token_verify_result.0.uid).await {
                                    Ok(user) => user,
                                    Err(e) => return Err(e),
                                };
                                let new_id_token = match IDToken::new(&user).sign() {
                                    Ok(token) => token,
                                    Err(_) => "".to_string(),
                                };

                                let new_refresh_token = match RefreshToken::new(&token_verify_result.0.uid).sign() {
                                    Ok(token) => token,
                                    Err(_) => "".to_string(),
                                };

                                // encrypt the new tokens
                                let new_id_token_encrypted = Encryption::encrypt_data(&new_id_token, &dek_data.dek);
                                let new_refresh_token_encrypted = Encryption::encrypt_data(&new_refresh_token, &dek_data.dek);

                                match collection_session
                                    .update_one(
                                        doc! {
                                            "uid": encrypted_id,
                                            "id_token": encrypted_id_token,
                                            "refresh_token": encrypted_refresh_token,
                                            "is_revoked": false,
                                        },
                                        doc! {
                                            "$set": {
                                                "id_token": new_id_token_encrypted,
                                                "refresh_token": new_refresh_token_encrypted,
                                                "updated_at": DateTime::now(),
                                            }
                                        },
                                        None,
                                    )
                                    .await
                                {
                                    Ok(_) => return Ok((new_id_token, new_refresh_token)),
                                    Err(e) => return Err(Error::ServerError { 
                                        message: e.to_string(),
                                    }),
                                };
                            } else {
                                match Self::revoke_all(&mongo_client, &token_verify_result.0.uid).await {
                                    Ok(_) => return Err(Error::SessionExpired {
                                        message: "Invalid token".to_string(),
                                    }),
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                        Err(e) => return Err(Error::ServerError {
                            message: e.to_string(),
                        }),
                    };
                } else {
                    return Err(Error::ActiveSessionExists {
                        message: "Active Session already exists".to_string(),
                    });
                }
            } 
            Err(e) => {
                return Err(e);
            }
        };
    }

    pub async fn get_all_from_uid(
        mongo_client: &Client,
        uid: &str,
    ) -> Result<Vec<SessionResponse>> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        let dek_data = match Dek::get(mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let encrypted_uid = Encryption::encrypt_data(uid, &dek_data.dek);

        let mut cursor = collection_session
            .find(
                doc! {
                    "uid": encrypted_uid,
                    "is_revoked": false,
                },
                None,
            )
            .await
            .unwrap();

        let mut sessions_res: Vec<SessionResponse> = Vec::new();
        while let Some(session) = cursor.next().await {
            match session {
                Ok(data) => {
                    let decrypted_session = data.decrypt(&dek_data.dek);
                    match IDToken::verify(&decrypted_session.id_token) {
                        Ok(token) => {
                            println!("{:?}", token);
                            sessions_res.push(SessionResponse {
                                uid: decrypted_session.uid,
                                email: decrypted_session.email,
                                user_agent: decrypted_session.user_agent,
                                is_revoked: decrypted_session.is_revoked,
                                created_at: decrypted_session.created_at,
                                updated_at: decrypted_session.updated_at,
                            });
                        }
                        Err(_) => continue,
                    }
                }
                Err(e) => {
                    return Err(Error::ServerError {
                        message: e.to_string(),
                    })
                }
            }
        }
        Ok(sessions_res)
    }

    pub async fn revoke_all(mongo_client: &Client, uid: &str) -> Result<()> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .update_many(doc! {"uid": uid}, doc! {"$set": {"is_revoked": true}}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn revoke(id_token: &str, refresh_token: &str, mongo_client: &Client) -> Result<()> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .update_one(
                doc! {"id_token": id_token, "refresh_token": refresh_token },
                doc! {"$set": {"is_revoked": true}},
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn delete(id_token: &str, refresh_token: &str, mongo_client: &Client) -> Result<()> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .delete_one(
                doc! {"id_token": id_token, "refresh_token": refresh_token },
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn delete_all(mongo_client: &Client, uid: &str) -> Result<()> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .delete_many(doc! {"uid": uid}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }
}
