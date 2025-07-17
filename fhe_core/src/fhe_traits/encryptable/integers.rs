use crate::configs::errors::FheError;
use tfhe::prelude::FheTryEncrypt;
use tfhe::zk::{CompactPkePublicParams, ZkComputeLoad};
use tfhe::{
    ClientKey, CompactPublicKey, FheInt64, FheUint64, ProvenCompactFheInt64, ProvenCompactFheUint64,
};
//type FheError = Box<dyn std::error::Error>;

/// Define Encryptable trait for integer encryption.
///
/// Valid type that implements Encryptable trait can be encrypted to
/// FheType with a client key. Valid type now temporarily supports integers
/// such as i64, u64. The output is FheType corresponding to the input.
/// Example:
///     i64 -> FheInt64
///     u64 -> FheUint64
///
/// Supported Type: Integer
pub trait Encryptable {
    type Output;

    fn val_encrypt(&self, client_key: &ClientKey) -> Result<Self::Output, FheError>
    where
        Self: Sized;

    // convert to json value
    fn to_json_value(&self) -> serde_json::Value;
}

/// Define ProvenEncryptable trait for integer encryption with zk.
///
/// Valid type that implements ProvenEncryptable trait can be encrypted to
/// ProvenFheType with a public zk params and a public key. Server can validate
/// the encrypted value before conducting the operation by using public zk params
/// and public key. Valid type now temporarily supports integers such as i64, u64.
/// The output is ProvenFheType corresponding to the input.
/// Example:
///    i64 -> ProvenCompactFheInt64
///    u64 -> ProvenCompactFheUint64
///
/// Supported Type: Integer
pub trait ProvenEncryptable {
    type ProvenOutput;
    fn proven_encrypt(
        &self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Result<Self::ProvenOutput, FheError>
    where
        Self: Sized;

    // convert to json value
    fn to_json_value(&self) -> serde_json::Value;
}

/// Implement Encryptable trait using macro_rules
///
/// Input Type: i64, u64
/// Output Type: FheInt64, FheUint64
///
/// Example:
/// ```no_run
/// impl_encryptable!(i64, FheInt64);
/// impl_encryptable!(u64, FheUint64);
/// ```
macro_rules! impl_encryptable {
    ($t:ty, $fhe_ty:ty) => {
        impl Encryptable for $t {
            type Output = $fhe_ty;

            fn val_encrypt(&self, client_key: &ClientKey) -> Result<Self::Output, FheError> {
                let encrypted_res = FheTryEncrypt::try_encrypt(*self, client_key);
                if let Ok(encrypted) = encrypted_res {
                    Ok(encrypted)
                } else {
                    encrypted_res.map_err(|e| FheError::EncryptionError(e.to_string()))
                }
            }

            fn to_json_value(&self) -> serde_json::Value {
                serde_json::Value::Number(serde_json::Number::from(*self))
            }
        }
    };
}

/// Implement ProvenEncryptable trait using macro_rules
///
/// Input Type: i64, u64
/// Output Type: ProvenCompactFheInt64, ProvenCompactFheUint64
///
/// Example:
/// ```no_run
/// impl_proven_encryptable!(i64, ProvenCompactFheInt64);
/// ```

macro_rules! impl_proven_encryptable {
    ($t:ty, $proven_fhe_ty:ty) => {
        impl ProvenEncryptable for $t {
            type ProvenOutput = $proven_fhe_ty;
            fn proven_encrypt(
                &self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Result<Self::ProvenOutput, FheError> {
                let encrypted_res = <$proven_fhe_ty>::try_encrypt(
                    *self,
                    public_zk_params,
                    &public_key,
                    ZkComputeLoad::Proof,
                );
                if let Ok(encrypted) = encrypted_res {
                    Ok(encrypted)
                } else {
                    encrypted_res.map_err(|e| FheError::EncryptionError(e.to_string()))
                }
            }

            fn to_json_value(&self) -> serde_json::Value {
                serde_json::Value::Number(serde_json::Number::from(*self))
            }
        }
    };
}

// Use the macro to implement Encryptable for i64 and u64
impl_encryptable!(i64, FheInt64);
impl_encryptable!(u64, FheUint64);
impl_proven_encryptable!(i64, ProvenCompactFheInt64);
impl_proven_encryptable!(u64, ProvenCompactFheUint64);
