use fhe_core::apis::base64;

#[neon::export]
pub fn encode_fhe_value(value: Vec<u8>) -> String {
    base64::encode_vec_u8(&value)
}

#[neon::export]
pub fn decode_fhe_value(value: String) -> Vec<u8> {
    base64::decode_vec_u8(&value)
}
