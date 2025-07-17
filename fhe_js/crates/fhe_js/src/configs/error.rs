use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Conversion error: {0}")]
    JsObjectConversionError(String),
}
