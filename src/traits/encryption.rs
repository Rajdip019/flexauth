use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::utils::encryption_utils::Encryption;

pub trait Encrypt {
    fn encrypt(&self, key: &str) -> Self;
}

impl<T> Encrypt for T
where
    T: Serialize + DeserializeOwned,
{
    fn encrypt(&self, key: &str) -> Self {
        // Serialize the object to JSON
        let json_str = serde_json::to_string(self).unwrap();

        // Deserialize the JSON string into a serde_json::Value
        let mut value: Value = serde_json::from_str(&json_str).unwrap();

        // Encrypt the keys and values recursively
        encrypt_value(&mut value, key);

        // Deserialize the serde_json::Value back to the original object
        serde_json::from_value(value).unwrap()
    }
}

// Recursive function to encrypt object keys and values
fn encrypt_value(value: &mut Value, key: &str) {
    match value {
        Value::String(s) => {
            // Encrypt string values
            *s = Encryption::encrypt_data(s, key);
        }
        Value::Object(map) => {
            // Check if this is a special MongoDB type, or if the key is "uid"
            let special_keys = [
                "$oid", "$date", "$numberLong", "$binary", "$timestamp", "$regex",
                "$symbol", "$code", "$codeWithScope", "$minKey", "$maxKey",
                "$undefined", "$null", "$numberInt", "$numberDouble", "$numberDecimal"
            ];

            for (k, v) in map.iter_mut() {
                if special_keys.contains(&k.as_str()) || k == "uid" {
                    continue;
                }
                encrypt_value(v, key);
            }
        }
        _ => {}
    }
}
