use crate::utils::base64;

/// Encode a Vec<u8> to a base64 string
pub fn encode_vec_u8(input: &Vec<u8>) -> String {
    base64::encode_vec_u8(input)
}

/// Decode a base64 string to a Vec<u8>
pub fn decode_vec_u8(input: &str) -> Vec<u8> {
    base64::decode_vec_u8(input).unwrap()
}

/// Encode a string to a base64 string
pub fn encode(input: &str) -> String {
    base64::encode(input)
}

/// Decode a base64 string to a string
pub fn decode(input: &str) -> String {
    base64::decode(input).unwrap()
}
