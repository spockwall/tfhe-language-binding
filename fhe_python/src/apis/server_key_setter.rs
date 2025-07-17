use fhe_core::configs::typing::SerialServerKey;
use fhe_core::fhe_traits::serializable::KeySerializable;
use fhe_core::tfhe::{set_server_key, CompressedServerKey, ServerKey};
use pyo3::prelude::*;

#[pyclass]
pub struct ServerKeySetter {}

#[pymethods]
impl ServerKeySetter {
    #[new]
    pub fn new() -> Self {
        ServerKeySetter {}
    }
    fn decompress_server_key(&self, server_key: SerialServerKey) -> Vec<u8> {
        let compressed_sks = CompressedServerKey::try_deserialize(&server_key).unwrap();
        let decompressed_sks = compressed_sks.decompress();
        decompressed_sks.try_serialize().unwrap().clone()
    }

    pub fn set_server_key(&self, server_key: SerialServerKey) {
        let server_key = ServerKey::try_deserialize(&server_key).unwrap();
        set_server_key(server_key);
    }
}
