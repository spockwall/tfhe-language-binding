//! This module provides python api for FHE operations
//! Input and output are serialized as Vec<u8> and String

use fhe_core::apis::fhe_ops::*;
use fhe_core::configs::typing::FheType;

/// Create python api for binary operations as String being data type
///
/// Input arguments:
///     a: Vec<u8> - first operand
///     b: Vec<u8> - second operand
///     data_type: String (see configs/typing.rs for more details)
///
/// Returns:
///     Vec<u8> - serialized result of the operation
#[macro_export]
macro_rules! impl_binary_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
            let fhe_type = FheType::from_str(&data_type);
            if let Ok(ty) = fhe_type {
                ($method)(&a, &b, &ty).expect("Failed to perform binary operation")
            } else {
                panic!("Failed to parse data type: {}", data_type)
            }
        }
    };
}

/// Create python api for unary operations as String being data type
///
/// Input arguments:
///     a: Vec<u8> - operand
///     data_type: String (see configs/typing.rs for more details)
///
/// Returns:
///     Vec<u8> - serialized result of the operation
///
macro_rules! impl_unary_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(a: Vec<u8>, data_type: String) -> Vec<u8> {
            let fhe_type = FheType::from_str(&data_type);
            if let Ok(ty) = fhe_type {
                ($method)(&a, &ty).expect("Failed to perform unary operation")
            } else {
                panic!("Failed to parse data type: {}", data_type)
            }
        }
    };
}

#[neon::export]
pub fn add(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(add, fhe_add);
    add(a, b, data_type)
}

#[neon::export]
pub fn sub(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(sub, fhe_sub);
    sub(a, b, data_type)
}

#[neon::export]
pub fn mul(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(mul, fhe_mul);
    mul(a, b, data_type)
}

#[neon::export]
pub fn div(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(div, fhe_div);
    div(a, b, data_type)
}

#[neon::export]
pub fn and(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(and, fhe_and);
    and(a, b, data_type)
}

#[neon::export]
pub fn or(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(or, fhe_or);
    or(a, b, data_type)
}

#[neon::export]
pub fn xor(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(xor, fhe_xor);
    xor(a, b, data_type)
}

#[neon::export]
pub fn shr(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(shr, fhe_shr);
    shr(a, b, data_type)
}

#[neon::export]
pub fn shl(a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_binary_fhe_ops!(shl, fhe_shl);
    shl(a, b, data_type)
}

#[neon::export]
pub fn not(a: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_unary_fhe_ops!(not, fhe_not);
    not(a, data_type)
}

#[neon::export]
pub fn neg(a: Vec<u8>, data_type: String) -> Vec<u8> {
    impl_unary_fhe_ops!(neg, fhe_neg);
    neg(a, data_type)
}
