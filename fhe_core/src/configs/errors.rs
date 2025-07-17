use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsmError {
    #[error("Invalid instruction line: {0}")]
    InvalidInstruction(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Register not found: {0}")]
    RegisterNotFound(String),

    #[error("Output error: {0}")]
    OutputError(String),
}

#[derive(Error, Debug)]
pub enum FheError {
    #[error("Invalid key")]
    InvalidKey,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Serialization error: {0}")]
    SerializeError(String),

    #[error("Deserialization error: {0}")]
    DeserializeError(String),

    #[error("Invalid FheType error: {0}")]
    InvalidFheType(String),
}
