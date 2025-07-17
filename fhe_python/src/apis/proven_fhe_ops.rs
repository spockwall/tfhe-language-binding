use crate::configs::typing::PyProvenFheType;
use fhe_core::apis::proven_fhe_ops::*;
use fhe_core::configs::typing::{SerialCompactPublicKey, SerialPublicZkParams};
use pyo3::prelude::*;

macro_rules! impl_binary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(
            a: Vec<u8>,
            b: Vec<u8>,
            data_type: PyProvenFheType,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> PyResult<Vec<u8>> {
            ($method)(&a, &b, &data_type.inner, &public_zk_param, &public_key)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
        }
    };
}

macro_rules! impl_unary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(
            a: Vec<u8>,
            data_type: PyProvenFheType,
            public_zk_param: &SerialPublicZkParams,
            public_key: &SerialCompactPublicKey,
        ) -> PyResult<Vec<u8>> {
            ($method)(&a, &data_type.inner, &public_zk_param, &public_key)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
        }
    };
}

#[pyclass]
pub struct ProvenFheOps {}

#[pymethods]
impl ProvenFheOps {
    #[new]
    pub fn new() -> Self {
        ProvenFheOps {}
    }

    pub fn add(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(add, proven_fhe_add);
        add(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn sub(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(sub, proven_fhe_sub);
        sub(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn mul(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(mul, proven_fhe_mul);
        mul(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn div(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(div, proven_fhe_div);
        div(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn and(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(and, proven_fhe_and);
        and(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn or(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(or, proven_fhe_or);
        or(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn xor(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(xor, proven_fhe_xor);
        xor(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn shr(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(shr, proven_fhe_shr);
        shr(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn shl(
        &self,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(shl, proven_fhe_shl);
        shl(a, b, data_type, &public_zk_param, &public_key)
    }

    pub fn not(
        &self,
        a: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_unary_py_fhe_ops!(not, proven_fhe_not);
        not(a, data_type, &public_zk_param, &public_key)
    }

    pub fn neg(
        &self,
        a: Vec<u8>,
        data_type: PyProvenFheType,
        public_zk_param: SerialPublicZkParams,
        public_key: SerialCompactPublicKey,
    ) -> PyResult<Vec<u8>> {
        impl_unary_py_fhe_ops!(neg, proven_fhe_neg);
        neg(a, data_type, &public_zk_param, &public_key)
    }
}
