#[cfg(test)]
mod fhe_tests {
    use fhe_core::fhe_traits::decryptable::Decryptable;
    use fhe_core::fhe_traits::encryptable::Encryptable;
    use tfhe::{generate_keys, ClientKey, ConfigBuilder, FheInt64};

    #[test]
    fn encrypt_and_decrypt_u64() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<u64> = vec![0, 11, 18446744073709551615, 34234];
        for input in input_vec {
            let encrypted = input.val_encrypt(&client_key).unwrap();
            let decrypted = encrypted.val_decrypt(&client_key);
            assert_eq!(input, decrypted);
        }
    }
    #[test]
    fn encrypt_and_decrypt_i64() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<i64> = vec![
            0,
            1123,
            -9_223_372_036_854_775_808,
            9_223_372_036_854_775_807i64,
        ];
        for input in input_vec {
            let encrypted = input.val_encrypt(&client_key).unwrap();
            let decrypted = encrypted.val_decrypt(&client_key);
            assert_eq!(input, decrypted);
        }
    }

    #[test]
    fn serialize_and_deserialize() {
        use fhe_core::fhe_traits::computable::Computable;
        use fhe_core::fhe_traits::serializable::KeySerializable;
        use tfhe::{set_server_key, ServerKey};
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        let client_key_serialized = client_key.try_serialize().unwrap();
        let server_key_serialized = server_key.try_serialize().unwrap();
        let client_key_deserialized: ClientKey =
            KeySerializable::try_deserialize(&client_key_serialized).unwrap();
        let server_key_deserialized: ServerKey =
            KeySerializable::try_deserialize(&server_key_serialized).unwrap();
        set_server_key(server_key_deserialized);
        let a: i64 = 123;
        let b: i64 = 456;
        let encrypted_a = a.val_encrypt(&client_key_deserialized).unwrap();
        let encrypted_b = b.val_encrypt(&client_key_deserialized).unwrap();

        let c = encrypted_a
            .add(&encrypted_b)
            .val_decrypt(&client_key_deserialized);
        assert_eq!(a + b, c);
    }

    #[test]
    fn encrypted_serialize() {
        use fhe_core::fhe_traits::serializable::FheValueSerializable;
        use tfhe::{generate_keys, ConfigBuilder};
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let a: i64 = 123;
        let encrypted_a = a.val_encrypt(&client_key).unwrap();
        let serialized_a = encrypted_a.try_serialize().unwrap();
        let deserialized_a = FheInt64::try_deserialize(&serialized_a).unwrap();
        let decrypted_a = deserialized_a.val_decrypt(&client_key);
        assert_eq!(a, decrypted_a);
    }
}
