use crate::configs::typing::PyFheType;
use crate::utils::conversion::py_dict_to_json;
use fhe_core::apis::http;
use fhe_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams, SerialServerKey,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Define fhe header in http request
/// 1. method: self-defined
/// 2. fhe-version: "tfhe:<version>"
/// 3. zk-experiment: bool
#[pyfunction]
#[pyo3(signature = (method, zk_experiment=None))]
pub fn create_fhe_header(method: &str, zk_experiment: Option<bool>) -> String {
    http::create_fhe_header(method, zk_experiment)
}

/// Encrypt body of http request with FHE
///
/// Input:
///     keys: List[Tuple[str, PyFheType]]
///         - list of keys in dict whose values to be enrypted with type: PyFheType
///     data: Dict - json data to be encrypted
#[pyfunction]
pub fn encrypt_fhe_body<'py>(
    keys: Vec<(String, PyFheType)>,
    data: Bound<'py, PyDict>,
    client_key: SerialClientKey,
) -> String {
    let keys = keys
        .iter()
        .map(|(k, v)| (k.clone(), v.inner.clone()))
        .collect();
    let data_json_str = py_dict_to_json(data).unwrap();
    http::encrypt_fhe_body(&keys, &data_json_str, &client_key)
}

#[pyfunction]
pub fn decrypt_fhe_body<'py>(
    keys: Vec<(String, PyFheType)>,
    data: Bound<'py, PyDict>,
    client_key: SerialClientKey,
) -> String {
    let keys = keys
        .iter()
        .map(|(k, v)| (k.clone(), v.inner.clone()))
        .collect();
    let data_json_str = py_dict_to_json(data).unwrap();
    http::decrypt_fhe_body(&keys, &data_json_str, &client_key)
}

#[pyfunction]
pub fn set_server_key_to_json<'py>(
    server_key: SerialServerKey,
    data: Bound<'py, PyDict>,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::set_server_key_to_json(&server_key, &data_json_str)
}

#[pyfunction]
pub fn set_public_key_to_json<'py>(
    public_key: SerialCompactPublicKey,
    data: Bound<'py, PyDict>,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::set_public_key_to_json(&public_key, &data_json_str)
}

#[pyfunction]
pub fn set_public_zk_params_to_json<'py>(
    public_zk_params: SerialPublicZkParams,
    data: Bound<'py, PyDict>,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::set_public_zk_params_to_json(&public_zk_params, &data_json_str)
}

#[pyfunction]
pub fn check_http_packet(packet: &str) -> PyResult<()> {
    http::check_http_packet(packet)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
}

#[pyfunction]
pub fn get_fhe_value_from_json<'py>(key: &str, data: Bound<'py, PyDict>) -> Vec<u8> {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::get_fhe_value_from_json(key, &data_json_str)
}
