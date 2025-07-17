use fhe_core::configs::zk_params;
use fhe_core::fhe_traits::serializable::ZkSerializable;
use pyo3::prelude::*;
/// Import the parameters from the shortint module
/// example:
///   use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40

/// Get PBS param depending on the message and carry
/// range of (msg, carry):
///    (1, 0),(1, 1),(1, 2),(1, 3),(1, 4),(1, 5),(1, 6),
///    (2, 0),(2, 1),(2, 2),(2, 3),(2, 4),(2, 5),(2, 6),
///    (3, 0),(3, 1),(3, 2),(3, 3),(3, 4),(3, 5)
#[pyfunction]
pub fn get_public_zk_params(msg: u8, carry: u8) -> Vec<u8> {
    let public_zk_params = zk_params::get_public_zk_params(msg, carry);
    let res = public_zk_params.try_serialize();
    match res {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    }
}
