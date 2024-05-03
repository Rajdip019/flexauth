use bson::DateTime;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use crate ::{
    errors::{Error, Result}, traits::encryption::Encrypt
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session{
    pub uid: String,
    pub email: String,
    pub id_token: String,
    pub refresh_token: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Session {
    pub fn new(uid: String, email: String, id_token: String, refresh_token: String) -> Self {
        Self {
            uid,
            email,
            id_token,
            refresh_token,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn encrypt_add(&self, mongo_client: &Client, key: &str) -> Result<Self> {
        let db = mongo_client.database("test");
        let collection_session: Collection<Session> = db.collection("sessions");

        let mut session = self.clone();
        let encrypted_session = session.encrypt(key);

        match collection_session.insert_one(encrypted_session, None).await {
            Ok(_) => Ok(session),
            Err(e) => Err(Error::ServerError {
                message: e.to_string(),
            }),
        }
    }
}