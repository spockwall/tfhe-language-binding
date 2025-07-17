use crate::configs::errors::FheError;
// key_serializable.rs
pub type SerialServerKey = Vec<u8>;
pub type SerialClientKey = Vec<u8>;
pub type SerialCompactPublicKey = Vec<u8>;
pub type SerialCompressedServerKey = Vec<u8>;
pub type SerialCompressedCompactPublicKey = Vec<u8>;
pub type SerialConfig = Vec<u8>;

// value
pub type SerialInt64 = Vec<u8>;
pub type SerialUint64 = Vec<u8>;

// fhe_value
pub type SerialFheInt64 = Vec<u8>;
pub type SerialFheUint64 = Vec<u8>;

// proven_fhe_value
pub type SerialProvenInt64 = Vec<u8>;
pub type SerialProvenUint64 = Vec<u8>;

// zk_serializable.rs
pub type SerialPublicZkParams = Vec<u8>;
pub type SerialPbsParams = Vec<u8>;

// other
pub type StringfiedJson = str;
pub type CompuationResult<T> = Result<T, &'static str>;

#[derive(Debug, Clone)]
pub enum FheType {
    Int64,
    Uint64,
}

#[derive(Debug, Clone)]
pub enum ProvenFheType {
    ProvenInt64,
    ProvenUint64,
}

impl FheType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FheType::Int64 => "Int64",
            FheType::Uint64 => "Uint64",
        }
    }

    pub fn from_str(s: &str) -> Result<FheType, FheError> {
        match s {
            "Int64" => Ok(FheType::Int64),
            "Uint64" => Ok(FheType::Uint64),
            _ => Err(FheError::InvalidFheType(format!("Invalid FheType: {}", s))),
        }
    }
}

#[rustfmt::skip]
impl ProvenFheType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProvenFheType::ProvenInt64 => "ProvenInt64",
            ProvenFheType::ProvenUint64 => "ProvenUint64",
        }
    }
    pub fn from_str(s: &str) -> Result<ProvenFheType, FheError> {
        match s {
            "ProvenInt64" => Ok(ProvenFheType::ProvenInt64),
            "ProvenUint64" => Ok(ProvenFheType::ProvenUint64),
            _ => Err(FheError::InvalidFheType(format!("Invalid ProvenFheType: {}", s))),
        }
    }
}
