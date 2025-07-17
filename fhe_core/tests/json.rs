#[cfg(test)]
mod file_ctl_tests {
    use fhe_core::configs::typing::FheType;
    use fhe_core::fhe_traits::decryptable::Decryptable;
    use fhe_core::utils::file_ctl::read_from_stream;
    use fhe_core::utils::json::{decrypt_json, encrypt_json, parse_json};
    use std::fs::File;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheInt64};
    const FILE_PATH: &str = "examples/json_files/param.json";
    #[test]
    fn encrypt_and_decrypt_json() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let mut plain_data = parse_json(&json);
        let keys = vec![
            ("a".to_string(), FheType::Int64),
            ("b".to_string(), FheType::Int64),
        ];
        let encrypted_data = encrypt_json(&keys, &mut plain_data, &client_key);
        let _ = decrypt_json(&keys, &encrypted_data, &client_key);
    }

    #[test]
    fn operate_on_ciphertext() {
        use fhe_core::fhe_traits::serializable::FheValueSerializable;
        use fhe_core::utils::json::get_fhe_value_from_json;
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        set_server_key(server_key);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let plain_data = parse_json(&json);
        let keys = vec![
            ("a".to_string(), FheType::Int64),
            ("b".to_string(), FheType::Int64),
        ];
        let encrypted_data = encrypt_json(&keys, &plain_data, &client_key);

        let encrypted_a = get_fhe_value_from_json("a", &encrypted_data);
        let encrypted_b = get_fhe_value_from_json("b", &encrypted_data);
        //let deserialized_a: FheJsonValue = bincode::deserialize(&encrypted_a).unwrap();

        let deserialized_a = FheInt64::try_deserialize(&encrypted_a).unwrap();
        let deserialized_b = FheInt64::try_deserialize(&encrypted_b).unwrap();
        //// Addition
        let encrypted_c = deserialized_a.clone() + deserialized_b.clone();
        let c = encrypted_c.val_decrypt(&client_key);
        print!("c: {}", c);
        assert_eq!(c, plain_data.get("c").unwrap().as_i64().unwrap());
    }
}
