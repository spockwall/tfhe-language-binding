pub mod configs {
    pub mod errors;
    pub mod instructions;
    pub mod typing;
    pub mod zk_params;
}

pub mod utils {
    pub mod base64;
    pub mod file_ctl;
    pub mod http;
    pub mod json;
    pub mod rle_compression;
}

pub mod fhe_traits {
    pub mod computable;
    pub mod decryptable;
    pub mod encryptable;
    pub mod serializable;
}

pub mod assembly {
    pub mod executor;
    pub mod parser;
}

pub mod apis {
    pub mod assembly;
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod proven_fhe_ops;
}

pub extern crate serde;
pub extern crate serde_json;
pub extern crate tfhe;
