use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::utils::encryption_utils::Encryption;

pub trait Decrypt {
    fn decrypt(&self, key: &str) -> Self;
}

impl<T> Decrypt for T
where
    T: Serialize + DeserializeOwned,
{
    fn decrypt(&self, key: &str) -> Self {
        // Serialize the object to JSON
        let json_str = serde_json::to_string(self).unwrap();

        // Deserialize the JSON string into a serde_json::Value
        let mut value: Value = serde_json::from_str(&json_str).unwrap();

        // Decrypt the keys and values recursively
        decrypt_value(&mut value, key);

        // Deserialize the serde_json::Value back to the original object
        serde_json::from_value(value).unwrap()
    }
}

// Recursive function to decrypt object keys and values
fn decrypt_value(value: &mut Value, key: &str) {
    match value {
        Value::String(s) => {
            // Decrypt string values
            *s = Encryption::decrypt_data(s, key);
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
                decrypt_value(v, key);
            }
        }
        _ => {}
    }
}
