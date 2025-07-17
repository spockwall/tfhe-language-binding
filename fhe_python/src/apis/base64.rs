use fhe_core::apis::base64;
use pyo3::prelude::*;

/// encode vec<u8> to base64
#[pyfunction]
pub fn encode_fhe_value(value: Vec<u8>) -> String {
    base64::encode_vec_u8(&value)
}

/// decode base64 to vec<u8>
#[pyfunction]
pub fn decode_fhe_value(value: &str) -> Vec<u8> {
    base64::decode_vec_u8(value)
}
