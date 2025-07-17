//! Neon library only supports for f64, so we need to convert
//! f64 to i64 and u64. Some precision will be lost in the process,
//! and only for 32 bit integer.

use fhe_core::configs::typing::*;
use fhe_core::fhe_traits::serializable::ValueSerializable;

#[neon::export]
pub fn from_i64(value: f64) -> SerialInt64 {
    let int_32 = value as i32 as i64;
    int_32
        .try_serialize()
        .expect(format!("Failed to serialize {}", value).as_str())
}

#[neon::export]
pub fn from_u64(value: f64) -> SerialUint64 {
    let int_32 = value as i32 as u64;
    int_32
        .try_serialize()
        .expect(format!("Failed to serialize {}", value).as_str())
}

#[neon::export]
pub fn to_i64(value: SerialInt64) -> f64 {
    i64::try_deserialize(&value).expect("Failed to deserialize i64") as f64
}

#[neon::export]
pub fn to_u64(value: SerialUint64) -> f64 {
    u64::try_deserialize(&value).expect("Failed to deserialize u64") as f64
}
