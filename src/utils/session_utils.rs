use axum::Json;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

use crate::models::user_model::User;

use super::encryption_utils::decrypt_data;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: String,
    email: String,
    role: String,
    exp: usize,
}

pub fn sign_jwt(user: &User, dek: &str) -> Result<String, Box<dyn Error>> {
    let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // decrypt the email and role using dek
    let email = decrypt_data(&user.email, &dek);
    let role = decrypt_data(&user.role, &dek);

    let header = Header::new(jwt::Algorithm::HS256);
    let token = jwt::encode(
        &header,
        &Claims {
            uid: user.uid.to_string(),
            email,
            role,
            exp: chrono::Utc::now().timestamp() as usize + 3600,
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
    let token_data: TokenData<Claims> = jwt::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ).expect("Failed to decode token");
     
    let token_data = token_data.claims;
    let user_id = token_data.uid;
    let email = token_data.email;
    let role = token_data.role;
    let exp = token_data.exp as i64;
    if exp < chrono::Utc::now().timestamp() {
        return Err("Token expired".into());
    }
    Ok(Json(json!({
        "uid": user_id,
        "email": email,
        "role": role,
        "exp": exp,
    })))
}
