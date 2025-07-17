//! This module provides python api for encryption and decryption
//! Input and output are serialized as Vec<u8> and PyFheType
use crate::configs::typing::{PyFheType, PyProvenFheType};
use fhe_core::apis::fhe::{decrypt, encrypt, proven_encrypt};
use fhe_core::configs::typing::{SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams};
use pyo3::prelude::*;

/// Fhe class for encryption and decryption. This class requires
/// client key for encryption and decryption. Public key is only
/// need when using asymmetric encryption, i.e., proven encryption.
#[pyclass]
pub struct Fhe {
    client_key: SerialClientKey,
    public_key: Option<SerialCompactPublicKey>,
}

#[pymethods]
impl Fhe {
    #[new]
    #[pyo3(signature = (client_key, public_key=None))]
    pub fn new(client_key: SerialClientKey, public_key: Option<SerialCompactPublicKey>) -> Self {
        let public_key = match public_key {
            Some(pk) => Some(pk.clone()),
            None => None,
        };
        Fhe {
            client_key: client_key.clone(),
            public_key: public_key.clone(),
        }
    }

    pub fn encrypt(&self, val: Vec<u8>, data_type: PyFheType) -> Vec<u8> {
        encrypt(&val, &self.client_key, &data_type.inner)
    }

    /// Encrypt with public key and zk parameters.
    /// Allowing server to verify the encryption.
    pub fn proven_encrypt(
        &self,
        val: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_params: SerialPublicZkParams,
    ) -> Vec<u8> {
        if let Some(pk) = &self.public_key {
            proven_encrypt(&val, pk, &public_zk_params, &data_type.inner)
        } else {
            panic!("Public key is required for proven encryption");
        }
    }

    pub fn decrypt(&self, val: Vec<u8>, data_type: PyFheType) -> Vec<u8> {
        decrypt(&val, &self.client_key, &data_type.inner)
    }
}
