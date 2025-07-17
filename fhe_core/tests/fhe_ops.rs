#[cfg(test)]
mod fhe_tests {
    use fhe_core::fhe_traits::computable::Computable;
    use fhe_core::fhe_traits::decryptable::Decryptable;
    use fhe_core::fhe_traits::encryptable::Encryptable;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder};

    #[test]
    fn fhe_operations() {
        use fhe_core::fhe_traits::encryptable::Encryptable;
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        set_server_key(server_key);

        // declare plain_text a, b
        let plain_a: i64 = 123;
        let plain_b: i64 = 456;
        let encrypted_a = plain_a.val_encrypt(&client_key).unwrap();
        let encrypted_b = plain_b.val_encrypt(&client_key).unwrap();

        // test fhe add
        let encrypted_c = encrypted_a.add(&encrypted_b);
        let decrypted_c = encrypted_c.val_decrypt(&client_key);
        assert_eq!(plain_a + plain_b, decrypted_c);

        // test fhe sub
        let encrypted_c = encrypted_a.sub(&encrypted_b);
        let decrypted_c = encrypted_c.val_decrypt(&client_key);
        assert_eq!(plain_a - plain_b, decrypted_c);

        // test fhe mul
        let encrypted_c = encrypted_a.mul(&encrypted_b);
        let decrypted_c = encrypted_c.val_decrypt(&client_key);
        assert_eq!(plain_a * plain_b, decrypted_c);
    }

    #[test]
    fn fhe_signed_int_encrypt() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        set_server_key(server_key);

        let plain_a: i64 = -123;
        let plain_b: i64 = 456;
        let encrypted_a = plain_a.val_encrypt(&client_key).unwrap();
        let encrypted_b = plain_b.val_encrypt(&client_key).unwrap();
        let encrypted_c = encrypted_a + encrypted_b;
        let decrypted_c = encrypted_c.val_decrypt(&client_key);
        assert_eq!(plain_a + plain_b, decrypted_c);
    }
}
