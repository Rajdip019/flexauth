use crate::{
    errors::{Error, Result},
    models::session_model::SessionResponse,
    traits::{decryption::Decrypt, encryption::Encrypt},
    utils::{
        email_utils::Email, encryption_utils::Encryption, session_utils::{IDToken, RefreshToken}
    },
};
use bson::{doc, DateTime};
use futures::StreamExt;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{dek::Dek, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub uid: String,
    pub session_id: String,
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
            session_id: Uuid::new_v4().to_string(),
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
        let db = mongo_client.database("auth");
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
                let db = mongo_client.database("auth");
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

    pub async fn refresh(
        mongo_client: &Client,
        uid: &str,
        session_id: &str,
        id_token: &str,
        refresh_token: &str,
        user_agent: &str,
    ) -> Result<(String, String)> {
        // verify refresh token 
        match RefreshToken::verify(&refresh_token) {
            Ok(_) => {}
            Err(e) => {
                match Self::revoke(&mongo_client, &session_id, &uid).await {
                    Ok(_) => return Err(e),
                    Err(err) => return Err(err),
                }
            },
        }
        match Self::verify(&mongo_client, &id_token).await {
            Ok(token_verify_result) => {
                if !token_verify_result.1 {
                    let db = mongo_client.database("auth");
                    let collection_session: Collection<Session> = db.collection("sessions");

                    let dek_data = match Dek::get(mongo_client, &token_verify_result.0.uid).await {
                        Ok(dek) => dek,
                        Err(e) => return Err(e),
                    };

                    let encrypted_uid = Encryption::encrypt_data(&token_verify_result.0.uid, &dek_data.dek);
                    let encrypted_id_token = Encryption::encrypt_data(&id_token, &dek_data.dek);
                    let encrypted_refresh_token =
                        Encryption::encrypt_data(&refresh_token, &dek_data.dek);
                    let encrypted_session_id = Encryption::encrypt_data(&session_id, &dek_data.dek);

                    match collection_session
                        .find_one(
                            doc! {
                                "uid": &encrypted_uid,
                                "session_id": &encrypted_session_id,
                                "is_revoked": false,
                            },
                            None,
                        )
                        .await
                    {
                        Ok(session) => {
                            match session {
                                Some(data) => {
                                    let decrypted_session = data.decrypt(&dek_data.dek);
                                    if decrypted_session.user_agent != user_agent {
                                        let user =  User::get_from_email(mongo_client, &decrypted_session.email).await.unwrap();
                                        Email::new(
                                            &user.name,
                                             &user.email,
                                              &"Unauthorized Login Attempt Detected",
                                         "We have detected an unauthorized login attempt associated with your account. For your security, we have taken action to protect your account.

                                            If you attempted to log in, please disregard this message. However, if you did not attempt to log in, we recommend taking the following steps:

                                            Immediately change your password to a strong, unique one.
                                            Review your account activity for any suspicious activity.
                                            If you have any concerns or questions, please don't hesitate to contact our support team.

                                            Stay safe and secure,
                                            FlexAuth Team").send().await;
                                         return Err(Error::InvalidUserAgent {
                                            message: "User Agent doesn't match with it's Session's User Agent".to_string(),
                                        })
                                    }
                                    if decrypted_session.id_token == id_token
                                        && decrypted_session.refresh_token == refresh_token
                                    {
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
                                                    "uid": encrypted_uid,
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
                                        match Self::revoke(&mongo_client, &session_id, &uid).await {
                                            Ok(_) => return Err(Error::InvalidToken {
                                                message: "Invalid token".to_string(),
                                            }),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                None => {
                                    return Err(Error::SessionExpired {
                                        message: "Invalid token".to_string(),
                                    });
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
                match Self::revoke(&mongo_client, &session_id, &uid).await {
                    Ok(_) => return Err(e),
                    Err(err) => return Err(err),
                }
            }
        };
    }

    pub async fn get_all(mongo_client: &Client) -> Result<Vec<SessionResponse>> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        // get all the sessions
        let mut cursor = collection_session.find(None, None).await.unwrap();

        let mut sessions = Vec::new();

        while let Some(session) = cursor.next().await {
            match session {
                Ok(data) => {
                    let dek_data = match Dek::get(mongo_client, &data.uid).await {
                        Ok(dek) => dek,
                        Err(e) => return Err(e),
                    };

                    let decrypted_session = data.decrypt(&dek_data.dek);

                    sessions.push(SessionResponse {
                        uid: decrypted_session.uid,
                        session_id: decrypted_session.session_id,
                        email: decrypted_session.email,
                        user_agent: decrypted_session.user_agent,
                        is_revoked: decrypted_session.is_revoked,
                        created_at: decrypted_session.created_at,
                        updated_at: decrypted_session.updated_at,
                    });
                }
                Err(_) => {
                    return Err(Error::ServerError {
                        message: "Failed to get session".to_string(),
                    });
                }
            }
        }
        // let collection_dek: Collection<Dek> = db.collection("deks");

        // let mut cursor_dek = collection_dek.find(None, None).await.unwrap();

        // let mut sessions = Vec::new();
        // let kek = env::var("SERVER_KEK").expect("Server Kek must be set.");

        // // iterate over the sessions and decrypt the data
        // while let Some(dek) = cursor_dek.next().await {
        //     let dek_data: Dek = match dek {
        //         Ok(data) => data.decrypt(&kek),
        //         Err(_) => {
        //             return Err(Error::ServerError {
        //                 message: "Failed to get DEK".to_string(),
        //             });
        //         }
        //     };

        //     // find the session in the sessions collection using the encrypted email to iterate over the sessions
        //     let cursor_session = collection_session
        //         .find_one(
        //             Some(doc! {
        //                 "uid": &dek_data.uid,
        //             }),
        //             None,
        //         )
        //         .await
        //         .unwrap();

        //     match cursor_session {
        //         Some(session) => {
        //             let session_data = session.decrypt(&dek_data.dek);

        //             sessions.push(SessionResponse {
        //                 uid: session_data.uid,
        //                 session_id: session_data.session_id,
        //                 email: session_data.email,
        //                 user_agent: session_data.user_agent,
        //                 is_revoked: session_data.is_revoked,
        //                 created_at: session_data.created_at,
        //                 updated_at: session_data.updated_at,
        //             });
        //         }
        //         None => {()}
        //     }
        // }

        // sort the sessions by created_at
        sessions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(sessions)
    }

    pub async fn get_all_from_uid(
        mongo_client: &Client,
        uid: &str,
    ) -> Result<Vec<SessionResponse>> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        let dek_data = match Dek::get(mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let mut cursor = collection_session
            .find(
                doc! {
                    "uid": uid,
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
                                session_id: decrypted_session.session_id,
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

    pub async fn get_details(mongo_client: &Client, uid: &str, session_id: &str) -> Result<SessionResponse> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        let dek_data = match Dek::get(mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let encrypted_session_id = Encryption::encrypt_data(session_id, &dek_data.dek);

        let session = match collection_session
            .find_one(doc! {"uid": &uid, "session_id": encrypted_session_id}, None)
            .await
        {
            Ok(session) => {
                match session {
                    Some(data) => {
                        let decrypted_session = data.decrypt(&dek_data.dek);
                        Ok(decrypted_session)
                    }
                    None => Err(Error::SessionNotFound  {
                        message: "Session not found".to_string(),
                    }),
                }
            }
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        };

        match session {
            Ok(data) => {
                Ok(SessionResponse {
                    uid: data.uid,
                    session_id: data.session_id,
                    email: data.email,
                    user_agent: data.user_agent,
                    is_revoked: data.is_revoked,
                    created_at: data.created_at,
                    updated_at: data.updated_at,
                })
            }
            Err(e) => Err(e),
        }
    } 

    pub async fn revoke_all(mongo_client: &Client, uid: &str) -> Result<()> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .update_many(doc! {"uid": &uid }, doc! {"$set": {"is_revoked": true}}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }

    pub async fn revoke(mongo_client: &Client, session_id: &str, uid: &str) -> Result<()> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        let dek_data = match Dek::get(mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let encrypted_session_id = Encryption::encrypt_data(session_id, &dek_data.dek);

        match collection_session
            .update_one(
                doc! {"session_id": encrypted_session_id},
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

    pub async fn delete(mongo_client: &Client, session_id: &str, uid: &str) -> Result<()> {
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        let dek_data = match Dek::get(mongo_client, uid).await {
            Ok(dek) => dek,
            Err(e) => return Err(e),
        };

        let encrypted_session_id = Encryption::encrypt_data(session_id, &dek_data.dek);

        match collection_session
            .delete_one(
                doc! { "session_id": encrypted_session_id },
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
        let db = mongo_client.database("auth");
        let collection_session: Collection<Session> = db.collection("sessions");

        match collection_session
            .delete_many(doc! {"uid": &uid }, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }
}
