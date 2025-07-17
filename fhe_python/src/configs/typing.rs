use fhe_core::configs::typing::{FheType, ProvenFheType};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyFheType {
    pub inner: FheType,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyProvenFheType {
    pub inner: ProvenFheType,
}

#[pymethods]
impl PyFheType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let fhe_type = FheType::from_str(s);
        if let Ok(ty) = fhe_type {
            Ok(PyFheType { inner: ty })
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to parse data type",
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pymethods]
impl PyProvenFheType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let proven_fhe_type = ProvenFheType::from_str(s);
        if let Ok(ty) = proven_fhe_type {
            Ok(PyProvenFheType { inner: ty })
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Failed to parse data type",
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pyfunction]
pub fn create_fhe_type(s: &str) -> PyResult<PyFheType> {
    PyFheType::from_str(s)
}

#[pyfunction]
pub fn create_proven_fhe_type(s: &str) -> PyResult<PyProvenFheType> {
    PyProvenFheType::from_str(s)
}
