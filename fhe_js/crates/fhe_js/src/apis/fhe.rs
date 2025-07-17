//! This module is for encryption and decryption. This class requires
//! client key for encryption and decryption. Public key is only
//! need when using asymmetric encryption, i.e., proven encryption.
use fhe_core::apis::fhe;
use fhe_core::configs::typing::{
    FheType, ProvenFheType, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};

/// Encrypt data using client key with type specified
#[neon::export]
pub fn encrypt(val: Vec<u8>, client_key: SerialClientKey, data_type: String) -> Vec<u8> {
    let fhe_type = FheType::from_str(&data_type);
    if let Ok(ty) = fhe_type {
        fhe::encrypt(&val, &client_key, &ty)
    } else {
        panic!("Failed to parse data type")
    }
}
/// Encrypt with public key and zk parameters.
/// Allowing server to verify the encryption.
#[neon::export]
pub fn proven_encrypt(
    val: Vec<u8>,
    public_key: SerialCompactPublicKey,
    public_zk_params: SerialPublicZkParams,
    data_type: String,
) -> Vec<u8> {
    let fhe_type = ProvenFheType::from_str(&data_type);
    if let Ok(ty) = fhe_type {
        fhe::proven_encrypt(&val, &public_key, &public_zk_params, &ty)
    } else {
        panic!("Failed to parse data type: {}", data_type)
    }
}

/// Decrypt the encrypted data with type specified.
#[neon::export]
pub fn decrypt(val: Vec<u8>, client_key: SerialClientKey, data_type: String) -> Vec<u8> {
    let fhe_type = FheType::from_str(&data_type);
    if let Ok(ty) = fhe_type {
        fhe::decrypt(&val, &client_key, &ty)
    } else {
        panic!("Failed to parse data type: {}", data_type)
    }
}
