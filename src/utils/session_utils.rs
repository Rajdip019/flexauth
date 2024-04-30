use axum::Json;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey, Header, Validation};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, env, fs};

use crate::{errors::Error, core::user::User};

#[derive(Debug, Serialize, Deserialize)]
struct IDTokenClaims {
    uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    token_type: String,
    data: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    data: Option<HashMap<String, String>>,
}

fn load_private_key() -> Result<Vec<u8>, Error> {
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
fn load_public_key() -> Result<Vec<u8>, Error> {
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

pub fn sign_jwt(user: &User) -> Result<String, Error> {
    let private_key = load_private_key()?;
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

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

    let claims: IDTokenClaims = IDTokenClaims {
        uid: user.uid.to_string(),
        iss: server_url,
        iat: chrono::Utc::now().timestamp() as usize,
        exp: chrono::Utc::now().timestamp() as usize + 3600,
        data: Some(
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
        token_type: "id_token".to_string(),
    };

    let token = match jwt::encode(&header, &claims, &encoding_key) {
        Ok(token) => token,
        Err(err) => {
            return Err(Error::IdTokenCreationError {
                message: err.to_string(),
            })
        }
    };

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Json<Value>, Error> {
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
    match jwt::decode::<IDTokenClaims>(&token, &decoding_key, &validation) {
        Ok(val) => {
            let token_data = val.claims;
            Ok(Json(json!({
                "valid": true,
                "data": token_data
            })))
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

// pub async fn generate_refresh_token(user: &User) -> Result<String, Box<dyn Error>> {
//     let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

//     let header = Header::new(jwt::Algorithm::HS256);
//     let token = jwt::encode(
//         &header,
//         &RefreshTokenClaims {
//             uid: user.uid.clone(),
//             iss: server_url,
//             iat: chrono::Utc::now().timestamp() as usize,
//             exp: chrono::Utc::now().timestamp() as usize + (3600 * 12), // 12h
//             data: None,
//         },
//         &EncodingKey::from_secret(secret.as_ref()),
//     )
//     .unwrap();

//     Ok(token)
// }
