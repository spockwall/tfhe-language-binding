use crate::configs::typing::{SerialPbsParams, SerialPublicZkParams};
use tfhe::shortint::ClassicPBSParameters;
use tfhe::zk::CompactPkePublicParams;

/// Define the Serializable trait for ZkType
///  
/// Values have types related to zk the implemented this trait
/// can be serialized into Vec<u8> and deserialized back into
/// the original value
///
/// Supported types:
///    - "ClassicPBSParameters"
///    - "CompactPkePublicParams"
pub trait ZkSerializable: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}

/// Implement Serializable trait using macro_rules
///
/// Input types:
///    - "ClassicPBSParameters"
///    - "CompactPkePublicParams"
/// Output types:
///   - "SerialPbsParams" (Vec<u8>)
///   - "SerialPublicZkParams" (Vec<u8>)
///
/// Example:
/// ```no_run
/// impl_zk_serializable!(ClassicPBSParameters, SerialPbsParams);
/// ```
macro_rules! impl_zk_serializable {
    ($t:ty, $s:ty) => {
        impl ZkSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }
            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_zk_serializable!(ClassicPBSParameters, SerialPbsParams);
impl_zk_serializable!(CompactPkePublicParams, SerialPublicZkParams);
