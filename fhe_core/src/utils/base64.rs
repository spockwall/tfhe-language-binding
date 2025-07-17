extern crate base64;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encode_vec_u8(input: &Vec<u8>) -> String {
    return STANDARD.encode(input);
}

pub fn decode_vec_u8(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    return STANDARD.decode(input);
}
pub fn encode(input: &str) -> String {
    return STANDARD.encode(input.as_bytes());
}

pub fn decode(input: &str) -> Result<String, base64::DecodeError> {
    match STANDARD.decode(input.as_bytes()) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);
            return Ok(decoded_str.to_string());
        }
        Err(err) => {
            panic!("decode error: {}", err)
        }
    }
}
