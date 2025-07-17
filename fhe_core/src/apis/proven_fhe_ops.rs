use crate::configs::typing::{
    CompuationResult, ProvenFheType, SerialCompactPublicKey, SerialPublicZkParams,
};
use crate::fhe_traits::computable::{ProvenComputable, ProvenShiftable};
use crate::fhe_traits::serializable::{
    FheValueSerializable, KeySerializable, ProvenFheValueSerializable, ZkSerializable,
};
use tfhe::zk::CompactPkePublicParams;
use tfhe::{CompactPublicKey, ProvenCompactFheInt64, ProvenCompactFheUint64};

/// API of binary operation functions for ProvenFheType
///
/// Input: api_name, operation_name
/// Output: api function
///
/// Example:
/// ```no_run
/// generate_binary_operation!(proven_fhe_add, add);
/// ```
macro_rules! generate_binary_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(
            a: &Vec<u8>,
            b: &Vec<u8>,
            data_type: &ProvenFheType,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> CompuationResult<Vec<u8>> {
            let public_zk_param = CompactPkePublicParams::try_deserialize(public_zk_param)
                .expect("Failed to deserialize public_zk_param");
            let public_key = CompactPublicKey::try_deserialize(public_key)
                .expect("Failed to deserialize public_key");
            match data_type {
                ProvenFheType::ProvenInt64 => {
                    let a = ProvenCompactFheInt64::try_deserialize(a).unwrap();
                    let b = ProvenCompactFheInt64::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b, &public_zk_param, &public_key); // Result is FheInt64
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                ProvenFheType::ProvenUint64 => {
                    let a = ProvenCompactFheUint64::try_deserialize(a).unwrap();
                    let b = ProvenCompactFheUint64::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b, &public_zk_param, &public_key); // Result is FheUint64
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
            }
        }
    };
}

/// API of unary operation functions for ProvenFheType
///
/// Input: api_name, operation_name
/// Output: api function
///
/// Example:
/// ```no_run
/// generate_unary_operation!(proven_fhe_neg, neg);
/// ```
macro_rules! generate_unary_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(
            a: &Vec<u8>,
            data_type: &ProvenFheType,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> CompuationResult<Vec<u8>> {
            let public_zk_param = CompactPkePublicParams::try_deserialize(public_zk_param)
                .expect("Failed to deserialize public_zk_param");
            let public_key = CompactPublicKey::try_deserialize(public_key)
                .expect("Failed to deserialize public_key");
            match data_type {
                ProvenFheType::ProvenInt64 => {
                    let a = ProvenCompactFheInt64::try_deserialize(a).unwrap();
                    let result = a.$op_method(&public_zk_param, &public_key);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                ProvenFheType::ProvenUint64 => {
                    let a = ProvenCompactFheUint64::try_deserialize(a).unwrap();
                    let result = a.$op_method(&public_zk_param, &public_key);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
            }
        }
    };
}

/// API of shift operation functions for ProvenFheType
///
/// Input: api_name, operation_name
/// Output: api function
///
/// Example:
/// ```no_run
/// generate_binary_shift_operation!(proven_fhe_shr, shr);
/// ```F
macro_rules! generate_binary_shift_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(
            a: &Vec<u8>,
            b: &Vec<u8>,
            data_type: &ProvenFheType,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> CompuationResult<Vec<u8>> {
            let public_zk_param = CompactPkePublicParams::try_deserialize(public_zk_param)
                .expect("Failed to deserialize public_zk_param");
            let public_key = CompactPublicKey::try_deserialize(public_key)
                .expect("Failed to deserialize public_key");
            match data_type {
                ProvenFheType::ProvenUint64 => {
                    let a = ProvenCompactFheUint64::try_deserialize(a).unwrap();
                    let b = ProvenCompactFheUint64::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b, &public_zk_param, &public_key);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                _ => {
                    panic!("Operation is not supported for Int64");
                }
            }
        }
    };
}
generate_binary_operation!(proven_fhe_add, add);
generate_binary_operation!(proven_fhe_sub, sub);
generate_binary_operation!(proven_fhe_mul, mul);
generate_binary_operation!(proven_fhe_div, div);
generate_binary_operation!(proven_fhe_rem, rem);
generate_binary_operation!(proven_fhe_and, and);
generate_binary_operation!(proven_fhe_or, or);
generate_binary_operation!(proven_fhe_xor, xor);
generate_binary_shift_operation!(proven_fhe_shr, shr);
generate_binary_shift_operation!(proven_fhe_shl, shl);
generate_unary_operation!(proven_fhe_neg, neg);
generate_unary_operation!(proven_fhe_not, not);
