use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit};

pub fn create_dek() -> String {
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

pub fn encrypt_data(data: &str, key_iv: &str) -> String {
    // split the key_iv into key and iv
    let key_iv_vec: Vec<&str> = key_iv.split('.').collect();
    let key_buff = Key::<Aes256Gcm>::from_slice(key_iv_vec[0].as_bytes().try_into().unwrap());
    let cipher = Aes256Gcm::new(key_buff);
    let cipher_text = cipher.encrypt(key_iv_vec[1].as_bytes().into(), data.as_ref()).unwrap();
    // convert the cipher_text to string
    return hex::encode(cipher_text);
}

pub fn decrypt_data(cipher_text: &str, key_iv: &str) -> String {
    // convert the cipher_text to bytes
    let cipher_text = hex::decode(cipher_text).unwrap();
    // split the key_iv into key and iv
    let key_iv_vec: Vec<&str> = key_iv.split('.').collect();
    let key_buff = Key::<Aes256Gcm>::from_slice(key_iv_vec[0].as_bytes().try_into().unwrap());
    let cipher = Aes256Gcm::new(key_buff);
    let data = cipher.decrypt(key_iv_vec[1].as_bytes().into(), cipher_text.as_ref()).unwrap();
    return String::from_utf8(data).unwrap();
}