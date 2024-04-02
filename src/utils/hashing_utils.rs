use sha256::digest;
use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHasher, SaltString
    },
    Argon2
};

pub struct HashedPassword {
    pub password: String,
    pub salt: String,
}

pub fn salt_and_hash_password(password: &str) -> HashedPassword {
    let input = String::from(password);
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash_and_salted_with_argon = argon2
        .hash_password(input.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let final_hash = digest(password_hash_and_salted_with_argon.to_string());
    return HashedPassword {
        password: final_hash,
        salt: salt.to_string(),
    };
}

pub fn verify_password(password: &str, salt: &str, hash: &str) -> bool {
    let input = String::from(password);
    // convert the salt to SaltString
    let salt_typed = SaltString::from_b64(salt).unwrap();
    let argon2 = Argon2::default();
    let password_hash_and_salted_with_argon = argon2
        .hash_password(input.as_bytes(), &salt_typed)
        .unwrap()
        .to_string();
    let final_hash = digest(password_hash_and_salted_with_argon.to_string());
    return final_hash == hash;
}

pub fn create_dek() -> String {
    let mut rng = OsRng;
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);
    return key.to_vec().iter().map(|b| format!("{:02x}", b)).collect::<String>();
}

pub fn create_iv() -> String {
    let mut rng = OsRng;
    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);
    return iv.to_vec().iter().map(|b| format!("{:02x}", b)).collect::<String>();
}