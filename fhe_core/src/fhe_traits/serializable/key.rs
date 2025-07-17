use crate::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialCompressedCompactPublicKey,
    SerialCompressedServerKey, SerialServerKey,
};
use tfhe::{
    ClientKey, CompactPublicKey, CompressedCompactPublicKey, CompressedServerKey, ServerKey,
};
/// Define the Serializable trait for KeyType
///
/// KeyType the implemented this trait can be serialized into
/// Vec<u8> and deserialized back into the original KeyType
///
/// Supported KeyType:
///     - "ClientKey"
///     - "ServerKey"
///     - "CompactPublicKey"
///     - "CompressedCompactPublicKey"
///     - "CompressedServerKey"
pub trait KeySerializable: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}

/// Implement Serializable trait using macro_rules
///
/// Input KeyType:
///     - "ClientKey"
///     - "ServerKey"
///     - "CompactPublicKey"
///     - "CompressedCompactPublicKey"
///     - "CompressedServerKey"
/// Output KeyType:
///    - "SerialClientKey" (Vec<u8>)
///    - "SerialServerKey" (Vec<u8>)
///    - "SerialCompactPublicKey" (Vec<u8>)
///    - "SerialCompressedCompactPublicKey" (Vec<u8>)
///    - "SerialCompressedServerKey" (Vec<u8>)
///
/// Example usage:
/// ```no_run
/// impl_key_serializable!(ClientKey, SerialClientKey);
/// ```
macro_rules! impl_key_serializable {
    ($t:ty, $s:ty) => {
        impl KeySerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }
            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_key_serializable!(ClientKey, SerialClientKey);
impl_key_serializable!(ServerKey, SerialServerKey);
impl_key_serializable!(CompressedCompactPublicKey, SerialCompressedCompactPublicKey);
impl_key_serializable!(CompressedServerKey, SerialCompressedServerKey);
impl_key_serializable!(CompactPublicKey, SerialCompactPublicKey);
impl_key_serializable!(tfhe::Config, Vec<u8>);
