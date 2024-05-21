use bson::{doc, DateTime};
use mongodb::{Client, Collection};

use crate::{
    core::{dek::Dek, session::Session, user::User},
    errors::{Error, Result},
    models::auth_model::{SessionResponseForSignInOrSignUp, SignInOrSignUpResponse},
    utils::{encryption_utils::Encryption, password_utils::Password},
};

pub struct Auth;

impl Auth {
    pub async fn sign_up(
        mongo_client: &Client,
        name: &str,
        email: &str,
        role: &str,
        password: &str,
        user_agent: &str,
    ) -> Result<SignInOrSignUpResponse> {
        let db = mongo_client.database("auth");

        let collection: Collection<User> = db.collection("users");
        let cursor = collection
            .find_one(
                Some(doc! {
                    "email": email
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
        let user = match User::new(name, email, role, password)
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

        let session = match Session::new(&user, user_agent)
            .encrypt_add(&mongo_client, &dek)
            .await
        {
            Ok(session) => session,
            Err(e) => return Err(e),
        };

        Ok(SignInOrSignUpResponse {
            message: "Signup successful".to_string(),
            uid: user.uid,
            name: user.name,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            email_verified: user.email_verified,
            is_active: user.is_active,
            session: SessionResponseForSignInOrSignUp {
                session_id: Encryption::encrypt_data(&session.session_id, &dek_data.dek),
                id_token: session.id_token,
                refresh_token: session.refresh_token,
            },
        })
    }

    pub async fn sign_in(
        mongo_client: &Client,
        email: &str,
        password: &str,
        user_agent: &str,
    ) -> Result<SignInOrSignUpResponse> {
        let user = match User::get_from_email(&mongo_client, email).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        // check if the user has a blocked_until date greater than the current date check in milliseconds from DateTime type
        match user.blocked_until {
            Some(blocked_until_time) => {
                let current_time = DateTime::now().timestamp_millis();
                if blocked_until_time.timestamp_millis() > current_time {
                    return Err(Error::UserBlocked {
                        message: "User is blocked".to_string(),
                    });
                }
            }
            None => {}
        }

        let dek_data = match Dek::get(&mongo_client, &user.uid).await {
            Ok(dek_data) => dek_data,
            Err(e) => return Err(e),
        };

        // verify the password
        if Password::verify_hash(password, &user.password) {
            let session = match Session::new(&user, &user_agent)
                .encrypt_add(&mongo_client, &dek_data.dek)
                .await
            {
                Ok(session) => session,
                Err(e) => return Err(e),
            };

            // make the failed login attempts to 0
            match User::reset_failed_login_attempt(&mongo_client, &user.email).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            let res = SignInOrSignUpResponse {
                message: "Signin successful".to_string(),
                uid: user.uid,
                name: user.name,
                email: user.email,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
                email_verified: user.email_verified,
                is_active: user.is_active,
                session: SessionResponseForSignInOrSignUp {
                    session_id: Encryption::encrypt_data(&session.session_id, &dek_data.dek),
                    id_token: session.id_token,
                    refresh_token: session.refresh_token,
                },
            };

            Ok(res)
        } else {
            match User::increase_failed_login_attempt(&mongo_client, &user.email).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            Err(Error::WrongCredentials {
                message: "Invalid credentials".to_string(),
            })
        }
    }

    pub async fn email_exists(mongo_client: &Client, email: &str) -> Result<bool> {
        let user = match User::get_from_email(&mongo_client, email).await {
            Ok(user) => user,
            Err(_) => return Ok(false),
        };

        Ok(user.email == email)
    }
}
