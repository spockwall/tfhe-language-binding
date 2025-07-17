use fhe_core::configs::typing::SerialPbsParams;
use fhe_core::fhe_traits::serializable::{KeySerializable, ZkSerializable};
use fhe_core::tfhe;
use std::option::Option;

#[neon::export]
pub fn create_config(params: Option<SerialPbsParams>) -> Result<Vec<u8>, neon::result::Throw> {
    let config: tfhe::Config = match params {
        Some(p) => {
            let params = tfhe::shortint::ClassicPBSParameters::try_deserialize(&p)
                .unwrap_or_else(|_| panic!("Invalid PBS params"));
            tfhe::ConfigBuilder::with_custom_parameters(params, None).build()
        }
        None => tfhe::ConfigBuilder::default().build(),
    };
    let config = config.try_serialize().unwrap();
    Ok(config)
}

#[neon::export]
pub fn generate_client_key<'a>(config: Vec<u8>) -> Result<Vec<u8>, neon::result::Throw> {
    let config = tfhe::Config::try_deserialize(&config).expect("Invalid input for config");
    let cks = tfhe::ClientKey::generate(config);
    let client_key = cks.try_serialize().unwrap();
    Ok(client_key)
}

#[neon::export]
pub fn generate_server_key<'a>(client_key: Vec<u8>) -> Result<Vec<u8>, neon::result::Throw> {
    let client_key =
        tfhe::ClientKey::try_deserialize(&client_key).expect("Invalid input for client key");
    let compressed_sks: tfhe::CompressedServerKey = tfhe::CompressedServerKey::new(&client_key);
    let server_key = compressed_sks.try_serialize().unwrap();
    Ok(server_key)
}

#[neon::export]
pub fn generate_public_key<'a>(client_key: Vec<u8>) -> Result<Vec<u8>, neon::result::Throw> {
    let client_key =
        tfhe::ClientKey::try_deserialize(&client_key).expect("Invalid input for client key");
    let public_key = tfhe::CompactPublicKey::new(&client_key);
    let public_key = public_key.try_serialize().unwrap();
    Ok(public_key)
}

#[neon::export]
pub fn generate_public_zk_params(
    max_num_message: f64, // should be usize,
    params: Option<SerialPbsParams>,
) -> Result<Vec<u8>, neon::result::Throw> {
    println!("Generating new public zk params");

    let params = params
        .map(|p| tfhe::shortint::ClassicPBSParameters::try_deserialize(&p))
        .unwrap_or_else(|| panic!("No PBS params provided to generate public zk params"))
        .unwrap_or_else(|_| panic!("Invalid PBS params"));

    let crs = tfhe::zk::CompactPkeCrs::from_shortint_params(params, max_num_message as usize)
        .unwrap_or_else(|_| panic!("Error: failed to generate public zk params"));

    let public_zk_params = crs.public_params();
    let public_zk_params = public_zk_params.try_serialize().unwrap();
    Ok(public_zk_params)
}
