use aes_gcm::{aead::OsRng, AeadCore, Aes256Gcm, KeyInit};

pub fn create_kek() -> String {
    let key = Aes256Gcm::generate_key(OsRng);
    // convert the key to hex string
    let hex_key = key.iter().map(|b| format!("{:02x}", b)).collect::<String>().chars().take(32).collect::<String>();
    let iv = Aes256Gcm::generate_nonce(&mut OsRng);
    // convert the iv to hex string
    let hex_iv = iv.iter().map(|b| format!("{:02x}", b)).collect::<String>().chars().take(12).collect::<String>();
    // connect the key and iv with . between them
    let key_iv = format!("{}.{}", hex_key, hex_iv);
    return key_iv;
}

pub fn generate_key_encryption_key() -> String {
    let key = create_kek();
    key
}

fn main() {
    let key = generate_key_encryption_key();
    println!("Key Encryption Key: {}", key)
}