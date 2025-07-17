use crate::configs::typing::FheType;
use crate::fhe_traits::encryptable::Encryptable;
use crate::fhe_traits::{decryptable::Decryptable, serializable::FheValueSerializable};
use crate::utils::base64;

use serde_json::{Map, Value};
use tfhe::{ClientKey, FheInt64, FheUint64};

/// Parse a JSON string into a JSON object
pub fn parse_json(json: &str) -> serde_json::Map<String, Value> {
    let res: Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => panic!("Error parsing JSON"),
    };
    let data = res.as_object().unwrap();
    return data.clone();
}

/// Encrypt a JSON object using the provided keys
/// The keys are used to encrypt the values of the JSON object
/// Encode the plaintext values by base64
/// args:
///   keys: &Vec<(String, FheType)> - The keys and types to the values to encrypt
///   data: Map<String, Value> - The JSON object to encrypt
///   client_key: &ClientKey - The secret key used for encryption
/// returns:
///   Map<String, Value> - The encrypted JSON object

pub fn encrypt_json(
    keys: &Vec<(String, FheType)>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
) -> Map<String, Value> {
    // Create new empty Map object that will store the encrypted values
    let mut encrypted_data: Map<String, Value> = serde_json::Map::new();

    for (key, data_type) in keys.iter() {
        if !data.contains_key(key) {
            panic!("Key not found in data");
        }
        let val: &Value = &data[key];
        let serial_data: Vec<u8>;
        match data_type {
            FheType::Int64 => {
                let val = val.as_i64().expect("Failed to parse value");
                let fhe_val = val.val_encrypt(client_key).expect("Failed to encrypt");
                serial_data = FheInt64::try_serialize(&fhe_val).expect("Failed to serialize");
            }
            FheType::Uint64 => {
                let val = val.as_u64().expect("Failed to parse value");
                let fhe_val = val.val_encrypt(client_key).expect("Failed to encrypt");
                serial_data = FheUint64::try_serialize(&fhe_val).expect("Failed to serialize");
            }
        }
        let encrypted_data_str = base64::encode_vec_u8(&serial_data);
        encrypted_data.insert(key.to_string(), encrypted_data_str.into());
    }
    // turn encrypted_data into a string
    return encrypted_data;
}

/// Decrypts a JSON object using the provided keys
/// The encrypted values of the JSON object are encdoed by base64
/// args:
///    keys: &Vec<(String, FheType)> - The keys and types to the values to decrypt
///    data: Map<String, Value> - The JSON object to decrypt
///    client_key: &ClientKey - The secret key used for decryption
/// returns:
///    Map<String, Value> - The decrypted JSON object
/// panic:
///   If the key is not found in the data
///   If the value type is not supported

pub fn decrypt_json(
    keys: &Vec<(String, FheType)>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
) -> Map<String, Value> {
    let mut decrypted_data: Map<String, Value> = serde_json::Map::new();
    for (key, value_type) in keys.iter() {
        let val_res = data.get(key).and_then(Value::as_str).expect("Key Error");
        let encrypted_data = base64::decode_vec_u8(val_res).expect("Failed to decode base64");

        // Try deserializing of FheInt64 and FheUint64
        match value_type {
            FheType::Int64 => {
                let fhe_val =
                    FheInt64::try_deserialize(&encrypted_data).expect("Failed to deserialize");
                let decrypted_val = fhe_val.val_decrypt(client_key);
                decrypted_data.insert(key.to_string(), decrypted_val.to_json_value());
            }
            FheType::Uint64 => {
                let fhe_val =
                    FheUint64::try_deserialize(&encrypted_data).expect("Failed to deserialize");
                let decrypted_val = fhe_val.val_decrypt(client_key);
                decrypted_data.insert(key.to_string(), decrypted_val.to_json_value());
            }
        }
    }
    return decrypted_data;
}

pub fn get_fhe_value_from_json(key: &str, encrypted_data: &Map<String, Value>) -> Vec<u8> {
    if !encrypted_data.contains_key(key) {
        panic!("Key not found in data");
    }
    let val = encrypted_data.get(key).unwrap();
    base64::decode_vec_u8(val.as_str().unwrap()).unwrap()
}
