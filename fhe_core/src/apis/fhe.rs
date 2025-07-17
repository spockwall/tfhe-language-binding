//! This fhe module is used to encrypt and decrypt values using the TFHE library.
use crate::configs::typing::{
    FheType, ProvenFheType, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::{Encryptable, ProvenEncryptable};
use crate::fhe_traits::serializable::{
    FheValueSerializable, KeySerializable, ProvenFheValueSerializable, ValueSerializable,
    ZkSerializable,
};
use tfhe::zk::CompactPkePublicParams;
use tfhe::{ClientKey, CompactPublicKey, FheInt64, FheUint64};

/// Encrypt a value using the client key and serialize the
/// result into a Vec<u8>. Type of value to be encrypted
/// is specified by data_type
///
/// Input: value, client_key, data_type
/// Output: encrypted value as Vec<u8>
///
/// Example:
/// ```no_run
/// let value = vec![210, 4, 0, 0, 0, 0, 0, 0]; // serialized 1234_i64
/// let encrypted = encrypt(&vec![1, 2, 3, 4], &client_key, &FheType::Int64);
/// ```
pub fn encrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &FheType) -> Vec<u8> {
    let deserialized_key = KeySerializable::try_deserialize(client_key).unwrap();
    match data_type {
        FheType::Int64 => {
            let deserialized_val = i64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .val_encrypt(&deserialized_key)
                .expect("Failed to encrypt i64");
            encrypted.try_serialize().expect("Failed to serialize i64")
        }
        FheType::Uint64 => {
            let deserialized_val = u64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .val_encrypt(&deserialized_key)
                .expect("Failed to encrypt u64");
            encrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}

/// Encrypt a value using public_key and public zk parameters
/// and serialize the result into a Vec<u8>. Type of value to be
/// encrypted is specified by data_type
///
/// Input: value, public_key, public_zk_params, data_type
/// Output: encrypted value as Vec<u8>
///
/// Example:
/// ```no_run
/// let value = vec![210, 4, 0, 0, 0, 0, 0, 0]; // serialized 1234_i64
/// let encrypted = proven_encrypt(&vec![1, 2, 3, 4], &public_key, &public_zk_params, &ProvenFheType::ProvenInt64);
/// ```
pub fn proven_encrypt(
    value: &Vec<u8>,
    public_key: &SerialCompactPublicKey,
    public_zk_params: &SerialPublicZkParams,
    data_type: &ProvenFheType,
) -> Vec<u8> {
    let deserialized_public_key = CompactPublicKey::try_deserialize(public_key).unwrap();
    let deserialized_public_zk_params =
        CompactPkePublicParams::try_deserialize(public_zk_params).unwrap();
    match data_type {
        ProvenFheType::ProvenInt64 => {
            let deserialized_val = i64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .proven_encrypt(&deserialized_public_zk_params, &deserialized_public_key)
                .expect("Failed to encrypt i64");
            encrypted.try_serialize().expect("Failed to serialize i64")
        }
        ProvenFheType::ProvenUint64 => {
            let deserialized_val = u64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .proven_encrypt(&deserialized_public_zk_params, &deserialized_public_key)
                .expect("Failed to encrypt u64");
            encrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}

/// Decrypt a FheType value using the client key and
/// serialize the result into a Vec<u8>. Type of value
/// to be decrypted is specified by data_type
///
/// Input: FheType value, client_key, data_type
/// Output: decrypted value as Vec<u8>
///
/// Example:
/// ```no_run
/// let res = decrypt(&encrypted, &client_key, &FheType::Int64);
/// ```
pub fn decrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &FheType) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerializable::try_deserialize(client_key).unwrap();
    match data_type {
        FheType::Int64 => {
            let deserialized_val = FheInt64::try_deserialize(value).unwrap();
            let decrypted = deserialized_val.val_decrypt(&deserialized_key);
            decrypted.try_serialize().expect("Failed to serialize i64")
        }
        FheType::Uint64 => {
            let deserialized_val = FheUint64::try_deserialize(value).unwrap();
            let decrypted = deserialized_val.val_decrypt(&deserialized_key);
            decrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}
