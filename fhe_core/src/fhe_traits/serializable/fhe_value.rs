use crate::configs::typing::{SerialFheInt64, SerialFheUint64};
use crate::fhe_traits::decryptable::Decryptable;
use tfhe::{FheInt64, FheUint64};

/// Define the Serializable trait for fhe values
///
/// FheType the implemented this trait can be serialized in to
/// Vec<u8> and deserialized back into the original FheType
///
/// Supported FheType: "FheInt", "FheUint"
pub trait FheValueSerializable: Decryptable + Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}

/// Implement Serializable trait using macro_rules
///
/// Input FheType: FheInt, FheUint
/// Output FheType:
///     - "SerialFheInt" (Vec<u8>)
///     - "SerialFheUint" (Vec<u8>)
/// Example:
/// ```no_run
/// impl_fhe_value_serializable!(FheInt64, SerialFheInt64);
/// ```
macro_rules! impl_fhe_value_serializable {
    ($t:ty, $s:ty) => {
        impl FheValueSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }

            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_fhe_value_serializable!(FheInt64, SerialFheInt64);
impl_fhe_value_serializable!(FheUint64, SerialFheUint64);
