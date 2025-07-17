use core::panic;
use fhe_core::apis::base64;
use fhe_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPbsParams, SerialPublicZkParams, SerialServerKey,
};
use fhe_core::fhe_traits::serializable::KeySerializable;
use fhe_core::fhe_traits::serializable::ZkSerializable;
use fhe_core::tfhe::{self, Config};
use project_root;
use pyo3::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::option::Option;
use tfhe::zk::CompactPkeCrs;

#[pyclass]
pub struct KeyGenerator {
    client_key: SerialClientKey,
    server_key: SerialServerKey,
    public_key: SerialCompactPublicKey,
    public_zk_params: SerialPublicZkParams,
    config: tfhe::Config,
}

#[pymethods]
impl KeyGenerator {
    #[new]
    // check if the new keys generation is needed
    #[pyo3(signature = (params=None))]
    pub fn new(params: Option<SerialPbsParams>) -> Self {
        let config: Config = match params {
            Some(p) => {
                let params = tfhe::shortint::ClassicPBSParameters::try_deserialize(&p)
                    .unwrap_or_else(|_| panic!("Invalid PBS params"));
                tfhe::ConfigBuilder::with_custom_parameters(params, None).build()
            }
            None => tfhe::ConfigBuilder::default().build(),
        };
        Self {
            client_key: Vec::new(),
            server_key: Vec::new(),
            public_key: Vec::new(),
            public_zk_params: Vec::new(),
            config,
        }
    }

    pub fn init_keys(&mut self) -> PyResult<()> {
        let res = self.load_keys();
        if res.is_err() {
            print!("fuck no key\n");
            let _res = self.generate_new_keys();
            self.save_keys()?;
        }
        Ok(())
    }

    pub fn generate_new_keys(&mut self) -> PyResult<()> {
        print!("Generating new keys\n");
        let cks = tfhe::ClientKey::generate(self.config);
        let compressed_sks: tfhe::CompressedServerKey = tfhe::CompressedServerKey::new(&cks);
        let public_key = tfhe::CompactPublicKey::new(&cks);
        self.client_key = cks.try_serialize().unwrap();
        self.server_key = compressed_sks.try_serialize().unwrap();
        self.public_key = public_key.try_serialize().unwrap();
        Ok(())
    }

    #[pyo3(signature = (max_num_message, params=None))]
    pub fn generate_public_zk_params(
        &mut self,
        max_num_message: usize,
        params: Option<SerialPbsParams>,
    ) -> PyResult<()> {
        println!("Generating new public zk params");

        let params = params
            .map(|p| tfhe::shortint::ClassicPBSParameters::try_deserialize(&p))
            .unwrap_or_else(|| panic!("No PBS params provided to generate public zk params"))
            .unwrap_or_else(|_| panic!("Invalid PBS params"));

        let crs = CompactPkeCrs::from_shortint_params(params, max_num_message)
            .unwrap_or_else(|_| panic!("Error: failed to generate public zk params"));

        let public_zk_params = crs.public_params();
        self.public_zk_params = public_zk_params.try_serialize().unwrap();
        Ok(())
    }

    pub fn get_client_key(&self) -> SerialClientKey {
        self.client_key.clone()
    }

    pub fn get_server_key(&self) -> SerialServerKey {
        // server key is compressed
        self.server_key.clone()
    }

    pub fn get_public_key(&self) -> SerialCompactPublicKey {
        // compact public key
        self.public_key.clone()
    }

    fn get_enc_path(&self) -> String {
        let file_name: String = String::from("fhe-language-binding-key.enc.json");
        let project_root = project_root::get_project_root();
        match project_root {
            Ok(val) => {
                let mut path = val.to_str().unwrap().to_string();
                path.push_str("/");
                path.push_str(&file_name);
                return path;
            }
            Err(_) => {
                panic!("Error: project root not found\n");
            }
        }
    }

    fn save_keys(&self) -> PyResult<()> {
        let file_path = self.get_enc_path();
        let mut file = File::create(file_path).expect("key.enc create failed");

        // json format
        let json = serde_json::json!({
            "client_key": base64::encode_vec_u8(&self.client_key),
            "server_key": base64::encode_vec_u8(&self.server_key),
            "public_key": base64::encode_vec_u8(&self.public_key),
        });

        // write the keys to the file
        file.write_all(json.to_string().as_bytes())
            .expect("key.enc write failed");

        print!("Keys saved successfully\n");
        Ok(())
    }

    fn load_keys(&mut self) -> PyResult<()> {
        let file_path = self.get_enc_path();
        // Read the entire file. This combines opening the file and reading it into a buffer.
        let buffer = std::fs::read(&file_path).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read key file: {}.",
                file_path
            ))
        })?;

        // Parse buffer directly as JSON. This avoids the explicit UTF-8 conversion.
        let data: serde_json::Value = serde_json::from_slice(&buffer).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse JSON from key file. Error: {e}"
            ))
        })?;

        // Extract keys, ensuring they exist and are strings.
        let client_key = data["client_key"].as_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'client_key' in JSON.")
        })?;

        let server_key = data["server_key"].as_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'server_key' in JSON.")
        })?;

        let public_key = data["public_key"].as_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'public_key' in JSON.")
        })?;

        // decompression and set key
        self.client_key = base64::decode_vec_u8(client_key);
        self.server_key = base64::decode_vec_u8(server_key);
        self.public_key = base64::decode_vec_u8(public_key);

        println!("Keys loaded successfully");
        Ok(())
    }
}
