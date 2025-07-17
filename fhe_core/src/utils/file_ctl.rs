use std::io::{Read, Result};

include!(concat!(env!("OUT_DIR"), "/tfhe_version.rs"));
pub fn read_from_stream<T: Read>(mut stream: T) -> Result<String> {
    let mut contents: String = String::new();
    stream.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_tfhe_version() -> &'static str {
    TFHE_VERSION
}
