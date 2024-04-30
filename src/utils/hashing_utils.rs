use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sha256::digest;

pub fn salt_and_hash_password(password: &str) -> String {
    let input = String::from(password);
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash_and_salted_with_argon = argon2
        .hash_password(input.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let final_hash = digest(password_hash_and_salted_with_argon.to_string());
    // return the hashed password and the salt connected with .
    return format!("{}.{}", final_hash, salt.to_string())
}

pub fn verify_password_hash(password: &str, hash: &str) -> bool {
    let input = String::from(password);
    // split the hash into hash and salt
    let hash_salt: Vec<&str> = hash.split('.').collect();
    // convert the salt to SaltString
    let salt_typed = SaltString::from_b64( hash_salt[1]).unwrap();
    let argon2 = Argon2::default();
    let password_hash_and_salted_with_argon = argon2
        .hash_password(input.as_bytes(), &salt_typed)
        .unwrap()
        .to_string();
    let final_hash = digest(password_hash_and_salted_with_argon.to_string());
    return final_hash == hash_salt[0];
}
