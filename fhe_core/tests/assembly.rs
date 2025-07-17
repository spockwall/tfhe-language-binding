#[cfg(test)]
mod tests {
    use fhe_core::assembly;
    use tfhe::prelude::*;

    #[test]
    fn assembly_execute() {
        // Parse the assembly code from input string
        let asm = assembly::parser::parse_asm_from_str(
            "VAR a r0\nVAR b r1\nNEG r0 r2\nADD r0 r1 r3\nADD r0 r3 r4\nOUT r4",
        )
        .unwrap();
        let mut args = std::collections::HashMap::new();

        // Initialize the FHE context
        let config = tfhe::ConfigBuilder::default().build();
        let (client_key, server_keys) = tfhe::generate_keys(config);
        let arg1 = tfhe::FheInt64::try_encrypt(12, &client_key);
        let arg2 = tfhe::FheInt64::try_encrypt(45, &client_key);
        tfhe::set_server_key(server_keys);

        // Create the arguments hashmap
        args.insert("a".to_string(), arg1.unwrap());
        args.insert("b".to_string(), arg2.unwrap());

        // Execute the assembly code
        let res = assembly::executor::execute_asm_i64(&asm, &args);
        let encrypted = res.unwrap();
        let decrypted: i64 = encrypted.decrypt(&client_key);
        print!("decrypted: {}", decrypted);
        //assert_eq!(decrypted, -321);
    }
}
