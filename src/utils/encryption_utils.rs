use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};

pub struct Encryption;

impl Encryption {
    pub fn encrypt_data(data: &str, key_iv: &str) -> String {
        // split the key_iv into key and iv
        let key_iv_vec: Vec<&str> = key_iv.split('.').collect();
        let key_buff = Key::<Aes256Gcm>::from_slice(key_iv_vec[0].as_bytes().try_into().unwrap());
        let cipher = Aes256Gcm::new(key_buff);
        let cipher_text = cipher
            .encrypt(key_iv_vec[1].as_bytes().into(), data.as_ref())
            .unwrap();
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
        let data = cipher
            .decrypt(key_iv_vec[1].as_bytes().into(), cipher_text.as_ref())
            .unwrap();
        return String::from_utf8(data).unwrap();
    }
}
