use pyo3::prelude::*;
pub mod apis {
    pub mod assembly;
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod key_generator;
    pub mod proven_fhe_ops;
    pub mod serializer;
    pub mod server_key_setter;
}
pub mod utils {
    pub mod conversion;
}

pub mod configs {
    pub mod typing;
    pub mod zk_params;
}

#[pymodule]
#[rustfmt::skip]
fn fhe(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<apis::fhe::Fhe>()?;
    m.add_class::<apis::fhe_ops::FheOps>()?;
    m.add_class::<apis::key_generator::KeyGenerator>()?;
    m.add_class::<apis::proven_fhe_ops::ProvenFheOps>()?;
    m.add_class::<apis::serializer::Serializer>()?;
    m.add_class::<apis::server_key_setter::ServerKeySetter>()?;
    m.add_function(wrap_pyfunction!(apis::assembly::execute_assembly, m)?)?;
    m.add_function(wrap_pyfunction!(apis::base64::encode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(apis::base64::decode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::create_fhe_header, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::encrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::decrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_server_key_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_public_key_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_public_zk_params_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::get_fhe_value_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(configs::zk_params::get_public_zk_params, m)?)?;
    m.add_function(wrap_pyfunction!(configs::typing::create_fhe_type, m)?)?;
    m.add_function(wrap_pyfunction!(configs::typing::create_proven_fhe_type, m)?)?;

    // Using wrap_enum! to expose the enum
    let fhe_value_enum = py.get_type_bound::<configs::typing::PyFheType>();
    let proven_fhe_value_enum = py.get_type_bound::<configs::typing::PyProvenFheType>();
    m.add("FheType", fhe_value_enum)?;
    m.add("ProvenFheType", proven_fhe_value_enum)?;
    Ok(())
}
