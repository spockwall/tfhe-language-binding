use tfhe::shortint::parameters;
use tfhe::zk::{CompactPkeCrs, CompactPkePublicParams};

pub fn get_public_zk_params(msg: u8, carry: u8) -> CompactPkePublicParams {
    let max_num_message = 1;
    let params = match (msg, carry) {
        (1, 0) => parameters::PARAM_MESSAGE_1_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 1) => parameters::PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 2) => parameters::PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 3) => parameters::PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 4) => parameters::PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 5) => parameters::PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 6) => parameters::PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 0) => parameters::PARAM_MESSAGE_2_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 1) => parameters::PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 2) => parameters::PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 3) => parameters::PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 4) => parameters::PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 5) => parameters::PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 6) => parameters::PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 0) => parameters::PARAM_MESSAGE_3_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 1) => parameters::PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 2) => parameters::PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 3) => parameters::PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 4) => parameters::PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 5) => parameters::PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        _ => panic!("Invalid parameters"),
    };
    let crs = CompactPkeCrs::from_shortint_params(params, max_num_message).unwrap();
    let public_zk_params = crs.public_params();
    return public_zk_params.clone();
}
