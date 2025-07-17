use fhe_core::apis::proven_fhe_ops::*;
use fhe_core::configs::typing::ProvenFheType;
use fhe_core::configs::typing::{SerialCompactPublicKey, SerialPublicZkParams};

macro_rules! impl_binary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(
            a: Vec<u8>,
            b: Vec<u8>,
            data_type: String,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> Vec<u8> {
            let proven_fhe_type = ProvenFheType::from_str(data_type.as_str());
            if let Ok(ty) = proven_fhe_type {
                ($method)(&a, &b, &ty, &public_zk_param, &public_key).unwrap()
            } else {
                panic!("Failed to parse data type: {}", data_type)
            }
        }
    };
}

macro_rules! impl_unary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(
            a: Vec<u8>,
            data_type: String,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> Vec<u8> {
            let proven_fhe_type = ProvenFheType::from_str(data_type.as_str());
            if let Ok(ty) = proven_fhe_type {
                ($method)(&a, &ty, &public_zk_param, &public_key).unwrap()
            } else {
                panic!("Failed to parse data type: {}", data_type)
            }
        }
    };
}

#[neon::export]
pub fn proven_add(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(add, proven_fhe_add);
    add(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_sub(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(sub, proven_fhe_sub);
    sub(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_mul(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(mul, proven_fhe_mul);
    mul(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_div(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(div, proven_fhe_div);
    div(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_and(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(and, proven_fhe_and);
    and(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_or(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(or, proven_fhe_or);
    or(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_xor(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(xor, proven_fhe_xor);
    xor(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_shr(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(shr, proven_fhe_shr);
    shr(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_shl(
    a: Vec<u8>,
    b: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_binary_py_fhe_ops!(shl, proven_fhe_shl);
    shl(a, b, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_not(
    a: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_unary_py_fhe_ops!(not, proven_fhe_not);
    not(a, data_type, &public_zk_param, &public_key)
}

#[neon::export]
pub fn proven_neg(
    a: Vec<u8>,
    data_type: String,
    public_zk_param: SerialPublicZkParams,
    public_key: SerialCompactPublicKey,
) -> Vec<u8> {
    impl_unary_py_fhe_ops!(neg, proven_fhe_neg);
    neg(a, data_type, &public_zk_param, &public_key)
}
