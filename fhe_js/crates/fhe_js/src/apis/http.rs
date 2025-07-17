use crate::configs::typing::JsFheType;
use crate::configs::typing::StringfiedJson;
use crate::utils::conversion::array_to_vec_string;
use fhe_core::apis::http;
use fhe_core::configs::typing::{
    FheType, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams, SerialServerKey,
};
use neon::prelude::*;

/// Define fhe header in http request
/// 1. method: self-defined
/// 2. fhe-version: "tfhe:<version>"
/// 3. zk-experiment: bool
#[neon::export]
pub fn create_fhe_header(method: String, zk_experiment: Option<bool>) -> String {
    http::create_fhe_header(method.as_str(), zk_experiment)
}

/// Encrypt body of http request with FHE
///
/// Input:
///     keys: Array[String]
///         - list of keys in dict whose values to be enrypted
///     data: Dict - json data to be encrypted
///     type: String - type of data to be encrypted
///     client_key: Vec<u8> - client key to encrypt data
#[neon::export]
pub fn encrypt_fhe_body<'cx>(
    cx: &mut FunctionContext<'cx>,
    keys: Handle<JsArray>, // Vec<String>
    data_type: JsFheType,
    data: StringfiedJson,
    client_key: SerialClientKey,
) -> String {
    let fhe_type = FheType::from_str(&data_type);
    let keys = array_to_vec_string(cx, keys);
    if let Ok(ty) = fhe_type {
        let keys_ty = keys.iter().map(|k| (k.clone(), ty.clone())).collect();
        http::encrypt_fhe_body(&keys_ty, &data, &client_key)
    } else {
        panic!("Failed to parse data type: {}", data_type)
    }
}

/// Encrypt body of http request with FHE
///
/// Input:
///     keys: Array[String]
///         - list of keys in dict whose values to be decrypted
///     data: Dict - json data to be decrypted
///     type: String - type of data to be decrypted
///     client_key: Vec<u8> - client key to decrypt data
#[neon::export]
pub fn decrypt_fhe_body<'cx>(
    cx: &mut FunctionContext<'cx>,
    keys: Handle<JsArray>,
    data_type: JsFheType,
    data: StringfiedJson,
    client_key: SerialClientKey,
) -> String {
    let fhe_type = FheType::from_str(&data_type);
    let keys = array_to_vec_string(cx, keys);
    if let Ok(ty) = fhe_type {
        let keys_ty = keys.iter().map(|k| (k.clone(), ty.clone())).collect();
        http::decrypt_fhe_body(&keys_ty, &data, &client_key)
    } else {
        panic!("Failed to parse data type: {}", data_type)
    }
}

#[neon::export]
pub fn set_server_key_to_json(server_key: SerialServerKey, data: StringfiedJson) -> String {
    http::set_server_key_to_json(&server_key, &data)
}

#[neon::export]
pub fn set_public_key_to_json(public_key: SerialCompactPublicKey, data: StringfiedJson) -> String {
    http::set_public_key_to_json(&public_key, &data)
}

#[neon::export]
pub fn set_public_zk_params_to_json(
    public_zk_params: SerialPublicZkParams,
    data: StringfiedJson,
) -> String {
    http::set_public_zk_params_to_json(&public_zk_params, &data)
}

#[neon::export]
pub fn check_http_packet(packet: String) -> Result<(), String> {
    http::check_http_packet(packet.as_str()).map_err(|e| e.to_string())
}

#[neon::export]
pub fn get_fhe_value_from_json(key: String, data: StringfiedJson) -> Vec<u8> {
    http::get_fhe_value_from_json(key.as_str(), &data)
}
