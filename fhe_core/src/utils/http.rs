use crate::configs::typing::FheType;
use crate::utils::base64;
use crate::utils::file_ctl::get_tfhe_version;
use crate::utils::json;
use serde_json::{Map, Value};
use tfhe;
use tfhe::ClientKey;

pub fn create_fhe_header(method: &str, zk_experimental: Option<bool>) -> String {
    let mut header = serde_json::Map::new();
    let version = get_tfhe_version().to_string();
    header.insert("fhe-method".to_string(), Value::String(method.to_string()));

    // add fhe-version header
    header.insert(
        "fhe-version".to_string(),
        Value::String(format!("tfhe:{}", version)),
    );

    // add zk-experiment header, using gzip to compress the body
    // turn true or false to string
    header.insert(
        "zk-experiment".to_string(),
        Value::String(match zk_experimental {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "false".to_string(),
        }),
    );

    return serde_json::to_string(&header).unwrap();
}

/// Encrypt the body of the HTTP packet using the provided keys
/// Currently, only the provided keys will be encrypted and left in the packet
/// The rest of columns will be dropped, the problem will be fix in the future
/// args:
///    keys: &Vec<(String, FheType)> - The keys to the values to encrypt
///    data: &str - The JSON object to encrypt which is stringified
///    client_key: Vec<u8> - The client key used for encryption
/// returns:
///    String - The encrypted JSON object which is stringified
///           - The encrypted value in encoded in base64
pub fn encrypt_fhe_body(
    keys: &Vec<(String, FheType)>,
    data: &str,
    client_key: &ClientKey,
) -> Map<String, Value> {
    let body = json::parse_json(data);
    let encrypted_body = json::encrypt_json(&keys, &body, &client_key);
    return encrypted_body;
}

/// Decrypt the body of the HTTP packet using the provided keys
/// Currently, only the provided keys will be decrypted and left in the packet
/// The rest of columns will be dropped, the problem will be fix in the future
/// args:
///   keys: &Vec<(String, FheType)> - The keys and types to the values to decrypt
///   data: &str - The JSON object to decrypt which is stringified
///   client_key: Vec<u8> - The client key used for decryption
/// returns:
///   String - The decrypted JSON object which is stringified
///          - The decrypted value in encoded in base64
pub fn decrypt_fhe_body(
    keys: &Vec<(String, FheType)>,
    data: &str,
    client_key: &ClientKey,
) -> Map<String, Value> {
    let body: serde_json::Map<String, serde_json::Value> = serde_json::from_str(data).unwrap();
    let decrypted_body = json::decrypt_json(&keys, &body, &client_key);
    return decrypted_body;
}

pub fn set_val_to_json(key_name: &str, key: &Vec<u8>, data: &str) -> String {
    let mut body = json::parse_json(data);
    body.insert(
        key_name.to_string(),
        serde_json::Value::String(base64::encode_vec_u8(&key)),
    );
    return serde_json::to_string(&body).unwrap();
}

pub fn parse_http_packet(packet: &str) -> (String, String) {
    // split the packet into header and body
    let res: Vec<&str> = packet.split("\r\n\r\n").collect();
    if res.len() == 2 {
        return (res[0].to_string(), res[1].to_string());
    }
    return ("".to_string(), "".to_string());
}

pub fn check_http_packet(packet: &str) -> Result<(), &str> {
    let (header, body) = parse_http_packet(packet);
    if header == "" || body == "" {
        return Err("Error parsing HTTP packet");
    }
    // check if fhe-method is present in the header
    let header = json::parse_json(&header);
    if !header.contains_key("fhe-method") {
        return Err("fhe-method not found in the header");
    }

    // check if server_key is present in the body
    let body = json::parse_json(&body);
    if !body.contains_key("server_key") {
        return Err("server_key not found in the body");
    }

    return Ok(());
}
