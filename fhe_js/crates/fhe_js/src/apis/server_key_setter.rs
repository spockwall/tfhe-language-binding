use fhe_core::configs::typing::SerialServerKey;
use fhe_core::fhe_traits::serializable::KeySerializable;
use fhe_core::tfhe;

#[neon::export]
pub fn set_server_key(server_key: SerialServerKey) {
    let compressed_sks = tfhe::CompressedServerKey::try_deserialize(&server_key).unwrap();
    let decompressed_sks = compressed_sks.decompress();
    tfhe::set_server_key(decompressed_sks);
}
