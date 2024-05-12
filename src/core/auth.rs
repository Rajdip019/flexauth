use bson::doc;
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
    ) -> Result<SignInOrSignUpResponse> {
        println!(">> HANDLER: add_user_handler called");

        let db = mongo_client.database("test");

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

        let session = match Session::new(&user, "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36").encrypt_add(&mongo_client, &dek).await {
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

        let dek_data = match Dek::get(&mongo_client, &user.uid).await {
            Ok(dek_data) => dek_data,
            Err(e) => return Err(e),
        };

        // verify the password
        if Password::verify_hash(password, &user.password) {
            let session = match Session::new(&user, &user_agent).encrypt_add(&mongo_client, &dek_data.dek).await {
            Ok(session) => session,
            Err(e) => return Err(e),
        };

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
            Err(Error::UserNotFound {
                message: "User not found".to_string(),
            })
        }
    }
}
