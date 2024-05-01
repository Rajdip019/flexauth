use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::utils::encryption_utils::decrypt_data;

pub trait Decrypt {
    fn decrypt(&mut self, key: &str) -> Self;
}

impl<T> Decrypt for T
where
    T: Serialize + DeserializeOwned,
{
    fn decrypt(&mut self, key: &str) -> Self {
        // Serialize the object to JSON
        let json_str = serde_json::to_string(self).unwrap();

        // Deserialize the JSON string into a serde_json::Value
        let mut value: Value = serde_json::from_str(&json_str).unwrap();

        // Encrypt the keys and values recursively
        decrypt_value(&mut value, key);

        // Deserialize the serde_json::Value back to the original object
        serde_json::from_value(value).unwrap()
    }
}

// Recursive function to encrypt object keys and values
fn decrypt_value(value: &mut Value, key: &str) {
    match value {
        Value::String(s) => {
            // Encrypt string values
            *s = decrypt_data(s, key);
        }
        Value::Object(map) => {
            // check if this is a ObjectId if yes do nothing return the same value
            if map.contains_key("$oid")
                || map.contains_key("$date")
                || map.contains_key("$numberLong")
                || map.contains_key("$binary")
                || map.contains_key("$timestamp")
                || map.contains_key("$regex")
                || map.contains_key("$symbol")
                || map.contains_key("$code")
                || map.contains_key("$codeWithScope")
                || map.contains_key("$minKey")
                || map.contains_key("$maxKey")
                || map.contains_key("$undefined")
                || map.contains_key("$null")
                || map.contains_key("$numberInt")
                || map.contains_key("$numberDouble")
                || map.contains_key("$numberDecimal")
            {
                return;
            }
            // Recursively encrypt keys and values of nested objects
            for (_, v) in map.iter_mut() {
                decrypt_value(v, key);
            }
        }
        _ => {}
    }
}
