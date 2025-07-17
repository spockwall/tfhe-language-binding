//! This module provides python api for FHE operations
//! Input and output are serialized as Vec<u8> and PyFheType

use crate::configs::typing::PyFheType;
use fhe_core::apis::fhe_ops::*;
use pyo3::prelude::*;

/// Create python api for binary operations as PyFheType being data type
///
/// Input arguments:
///     a: Vec<u8> - first operand
///     b: Vec<u8> - second operand
///     data_type: PyFheType (see configs/typing.rs for more details)
///
/// Returns:
///     Vec<u8> - serialized result of the operation
#[macro_export]
macro_rules! impl_binary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
            ($method)(&a, &b, &data_type.inner)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
        }
    };
}

/// Create python api for unary operations as PyFheType being data type
///
/// Input arguments:
///     a: Vec<u8> - operand
///     data_type: PyFheType (see configs/typing.rs for more details)
///     
/// Returns:
///     Vec<u8> - serialized result of the operation
///     
macro_rules! impl_unary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(a: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
            ($method)(&a, &data_type.inner)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
        }
    };
}
#[pyclass]
pub struct FheOps {}

#[pymethods]
impl FheOps {
    #[new]
    pub fn new() -> Self {
        FheOps {}
    }

    pub fn add(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(add, fhe_add);
        add(a, b, data_type)
    }

    pub fn sub(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(sub, fhe_sub);
        sub(a, b, data_type)
    }

    pub fn mul(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(mul, fhe_mul);
        mul(a, b, data_type)
    }

    pub fn div(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(div, fhe_div);
        div(a, b, data_type)
    }

    pub fn and(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(and, fhe_and);
        and(a, b, data_type)
    }

    pub fn or(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(or, fhe_or);
        or(a, b, data_type)
    }

    pub fn xor(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(xor, fhe_xor);
        xor(a, b, data_type)
    }

    pub fn shr(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(shr, fhe_shr);
        shr(a, b, data_type)
    }

    pub fn shl(&self, a: Vec<u8>, b: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_binary_py_fhe_ops!(shl, fhe_shl);
        shl(a, b, data_type)
    }

    pub fn not(&self, a: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_unary_py_fhe_ops!(not, fhe_not);
        not(a, data_type)
    }

    pub fn neg(&self, a: Vec<u8>, data_type: PyFheType) -> PyResult<Vec<u8>> {
        impl_unary_py_fhe_ops!(neg, fhe_neg);
        neg(a, data_type)
    }
}
