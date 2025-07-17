use tfhe::{prelude::*, ClientKey};

/// Define Decryptable trait for FheType decryption.
///
/// FheType that implements Decryptable trait can be decrypted
/// to the original value. The output is the original value
/// corresponding to the FheType.
///
/// Example:
///    FheInt64 -> i64
///    FheUint64 -> u64
pub trait Decryptable {
    type Output;

    fn val_decrypt(&self, client_key: &ClientKey) -> Self::Output
    where
        Self: Sized;
}

/// Implement Decryptable trait using macro_rules
///
/// Input FheType: FheInt64, FheUint64
/// Output Type: i64, u64
///
/// Example:
///    FheInt64 -> i64
///    FheUint64 -> u64
macro_rules! impl_decryptable {
    ($fhe_ty:ty, $t:ty) => {
        impl Decryptable for $fhe_ty {
            type Output = $t;

            fn val_decrypt(&self, client_key: &ClientKey) -> $t {
                (*self).decrypt(client_key)
            }
        }
    };
}

// Use the macro to implement Encryptable for i64
impl_decryptable!(tfhe::FheInt64, i64);
impl_decryptable!(tfhe::FheUint64, u64);
