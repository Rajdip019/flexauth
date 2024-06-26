use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey, Header, Validation};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

use crate::{core::user::User, errors::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct IDToken {
    pub uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    token_type: String,
    pub data: Option<HashMap<String, String>>,
}

pub fn load_private_key() -> Result<Vec<u8>, Error> {
    let private_key_content = fs::read("private_key.pem");
    let rsa = Rsa::private_key_from_pem(&private_key_content.unwrap()).unwrap();
    let private_key = PKey::from_rsa(rsa).unwrap();
    match private_key.private_key_to_pem_pkcs8() {
        Ok(key) => return Ok(key),
        Err(_) => {
            return Err(Error::PrivateKeyLoadError {
                message: "Error loading private key".to_string(),
            });
        }
    };
}

// Load public key from the private key
pub fn load_public_key() -> Result<Vec<u8>, Error> {
    let private_key_content = fs::read("private_key.pem");
    let rsa = Rsa::private_key_from_pem(&private_key_content.unwrap()).unwrap();
    let private_key = PKey::from_rsa(rsa).unwrap();
    match private_key.public_key_to_pem() {
        Ok(key) => return Ok(key),
        Err(_) => {
            return Err(Error::PublicKeyLoadError {
                message: "Error deriving public key from private key".to_string(),
            });
        }
    };
}

impl IDToken {
    pub fn new(user: &User) -> Self {
        let server_url =
            env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        Self {
            uid: user.uid.to_string(),
            iss: server_url,
            iat: chrono::Utc::now().timestamp() as usize,
            exp: chrono::Utc::now().timestamp() as usize + 3600, // 1h
            token_type: "id".to_string(),
            data : Some(
                [
                    ("display_name".to_string(), user.name.to_string()),
                    ("role".to_string(), user.role.to_string()),
                    ("is_active".to_string(), user.is_active.to_string()),
                    (
                        "is_email_verified".to_string(),
                        user.email_verified.to_string(),
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
        }
    }

    pub fn sign(&self) -> Result<String, Error> {
        let private_key = load_private_key()?;
        let header = Header::new(jwt::Algorithm::RS256);
        let encoding_key = match EncodingKey::from_rsa_pem(&private_key) {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Error creating decoding key: {}", err);
                return Err(Error::PublicKeyLoadError {
                    message: (err.to_string()),
                });
            }
        };

        match jwt::encode(&header, &self, &encoding_key) {
            Ok(token) => return Ok(token),
            Err(err) => {
                return Err(Error::IdTokenCreationError {
                    message: err.to_string(),
                })
            }
        };
    }

    pub fn verify(token: &str) -> Result<(Self, bool), Error> {
        let public_key = load_public_key()?;
        let validation = Validation::new(jwt::Algorithm::RS256);
        // Try to create a DecodingKey from the public key
        let decoding_key = match DecodingKey::from_rsa_pem(&public_key) {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Error creating decoding key: {}", err);
                return Err(Error::PublicKeyLoadError {
                    message: (err.to_string()),
                });
            }
        };
        // return false if the token is not valid
        match jwt::decode::<IDToken>(&token, &decoding_key, &validation) {
            Ok(val) => {
                let token_data = val.claims;
                Ok((token_data, true))
            }
            Err(e) => match e.kind() {
                // check if ExpiredSignature
                jwt::errors::ErrorKind::ExpiredSignature => {
                    // get token claims even if it is expired to check the data by decoding it with exp flag set to false
                    let mut validation = Validation::new(jwt::Algorithm::RS256);
                    validation.validate_exp = false;
                    match jwt::decode::<IDToken>(&token, &decoding_key, &validation) {
                        Ok(val) => {
                            let token_data = val.claims;
                            Ok((token_data, false))
                        }
                        Err(_) => {
                            return Err(Error::ServerError {
                                message: "Error decoding token".to_string(),
                            })
                        }
                    }
                }
                // check if InvalidSignature
                jwt::errors::ErrorKind::InvalidSignature => {
                    return Err(Error::SignatureVerificationError {
                        message: "Invalid signature".to_string(),
                    })
                }
                _ => {
                    return Err(Error::InvalidToken {
                        message: "Invalid token".to_string(),
                    })
                }
            },
        }
    }
    
}

// RefreshToken struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    scope: String,
    pub data: Option<HashMap<String, String>>,
}

impl RefreshToken {
    pub fn new(uid: &str) -> Self {
        let server_url =
            std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        Self {
            uid: uid.to_string(),
            iss: server_url,
            iat: chrono::Utc::now().timestamp() as usize,
            exp: chrono::Utc::now().timestamp() as usize + (3600 * 24 * 45), // 45 days
            scope: "get_new_id_token".to_string(),
            data: None,
        }
    }

    pub fn sign(&self) -> Result<String, Error> {
        let private_key = load_private_key()?;
        let header = Header::new(jwt::Algorithm::RS256);
        let encoding_key = match EncodingKey::from_rsa_pem(&private_key) {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Error creating decoding key: {}", err);
                return Err(Error::PublicKeyLoadError {
                    message: (err.to_string()),
                });
            }
        };

        match jwt::encode(&header, &self, &encoding_key) {
            Ok(token) => return Ok(token),
            Err(err) => {
                return Err(Error::RefreshTokenCreationError {
                    message: err.to_string(),
                })
            }
        };
    }

    pub fn verify(token: &str) -> Result<Self, Error> {
        let public_key = load_public_key()?;
        let validation = Validation::new(jwt::Algorithm::RS256);
        // Try to create a DecodingKey from the public key
        let decoding_key = match DecodingKey::from_rsa_pem(&public_key) {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Error creating decoding key: {}", err);
                return Err(Error::PublicKeyLoadError {
                    message: (err.to_string()),
                });
            }
        };
        // return false if the token is not valid
        match jwt::decode::<RefreshToken>(&token, &decoding_key, &validation) {
            Ok(val) => {
                let token_data = val.claims;
                Ok(token_data)
            }
            Err(e) => match e {
                // check if ExpiredSignature
                _ if e.to_string().contains("ExpiredSignature") => {
                    return Err(Error::ExpiredSignature {
                        message: "Expired signature".to_string(),
                    })
                }
                // check if InvalidSignature
                _ if e.to_string().contains("InvalidSignature") => {
                    return Err(Error::SignatureVerificationError {
                        message: "Invalid signature".to_string(),
                    })
                }
                _ => {
                    return Err(Error::InvalidToken {
                        message: "Invalid token".to_string(),
                    })
                }
            },
        }
    }
}