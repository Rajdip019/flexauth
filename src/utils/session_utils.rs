use axum::Json;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, env, error::Error};

use crate::models::user_model::User;

use super::encryption_utils::decrypt_data;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: String,
    iss: String,
    iat: usize,
    exp: usize,
    data: Option<HashMap<String, String>>,
}

pub fn sign_jwt(user: &User, dek: &str) -> Result<String, Box<dyn Error>> {
    let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // decrypt the email and role using dek
    // let email = decrypt_data(&user.email, &dek);
    let role = decrypt_data(&user.role, &dek);
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    let header = Header::new(jwt::Algorithm::HS256);
    let token = jwt::encode(
        &header,
        &Claims {
            uid: user.uid.clone(),
            iss: server_url,
            iat: chrono::Utc::now().timestamp() as usize,
            exp: chrono::Utc::now().timestamp() as usize + 3600,
            data: Some(
                [
                    ("display_name".to_string(), user.name.to_string()),
                    ("role".to_string(), role),
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
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Json<Value>, Box<dyn Error>> {
    let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let validation = Validation::new(jwt::Algorithm::HS256);
    // return false if the token is not valid
    match jwt::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(val) => {
            let token_data = val.claims;
            let user_id = token_data.uid;
            let iss = token_data.iss;
            let iat = token_data.iat as i64;
            let data = token_data.data.unwrap();
            let exp = token_data.exp as i64;
            Ok(Json(json!({
                "valid": true,
                "data": {
                    "uid": user_id,
                    "iss": iss,
                    "iat": iat,
                    "exp": exp,
                    "data": data
                }
            })))
        }
        Err(e) => return Err(Box::new(e)),
    }
}
