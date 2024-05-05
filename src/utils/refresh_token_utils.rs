use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    scope: String,
    data: Option<HashMap<String, String>>,
}

impl RefreshToken {
    pub fn new(uid: &str, scope: &str) -> Self {
        let server_url =
            std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        Self {
            uid: uid.to_string(),
            iss: server_url,
            iat: chrono::Utc::now().timestamp() as usize,
            exp: chrono::Utc::now().timestamp() as usize + (3600 * 24 * 30), // 30 days
            scope: scope.to_string(),
            data: None,
        }
    }

    pub fn sign(&self) -> String {
        let private_key = load_private_key().unwrap();
        let rsa = Rsa::private_key_from_pem(&private_key).unwrap();
        let private_key = PKey::from_rsa(rsa).unwrap();
        let mut claims = ClaimsSet::<RefreshToken> {
            registered: RegisteredClaims {
                issuer: Some(From::from(self.iss.clone())),
                subject: Some(From::from(self.uid.clone())),
                issued_at: Some(From::from(self.iat)),
                expiration: Some(From::from(self.exp)),
                not_before: None,
                ..Default::default()
            },
            private: self.clone(),
        };
        let header = Header {
            algorithm: Algorithm::RS256,
            ..Default::default()
        };
        encode(&header, &claims, &private_key).unwrap()
    }
}