use crate::configs::typing::PyFheType;
use fhe_core::apis::assembly;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
pub fn execute_assembly(
    assembly: &str,
    args: HashMap<String, Vec<u8>>,
    data_type: PyFheType,
) -> PyResult<Vec<u8>> {
    let res = assembly::execute_assembly(assembly, args, &data_type.inner);
    if let Err(e) = res {
        let err_msg = format!("{:?}", e);
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err_msg));
    }
    Ok(res.unwrap())
}
