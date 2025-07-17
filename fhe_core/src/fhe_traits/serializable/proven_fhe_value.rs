use crate::configs::typing::{SerialProvenInt64, SerialProvenUint64};
use tfhe::{ProvenCompactFheInt64, ProvenCompactFheUint64};

/// Define the Serializable trait for ProvenFheType
///
/// ProvenFheType the implemented this trait can be serialized
/// into Vec<u8> and deserialized back into the original ProvenFheType
///
/// Supported ProvenFheType:
///    - "ProvenCompactFheInt"
///    - "ProvenCompactFheUint"
pub trait ProvenFheValueSerializable: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}

/// Implement Serializable trait using macro_rules
///
/// Input ProvenFheType:
///     - "ProvenCompactFheInt"
///     - "ProvenCompactFheUint"
/// Output ProvenFheType:
///     - "SerialProvenInt" (Vec<u8>)
///     - "SerialProvenUint" (Vec<u8>)
///
/// Example:
/// ```no_run
/// impl_fhe_value_serializable!(ProvenCompactFheInt64, SerialProvenInt64);
/// ```
macro_rules! impl_fhe_value_serializable {
    ($t:ty, $s:ty) => {
        impl ProvenFheValueSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }

            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_fhe_value_serializable!(ProvenCompactFheInt64, SerialProvenInt64);
impl_fhe_value_serializable!(ProvenCompactFheUint64, SerialProvenUint64);
