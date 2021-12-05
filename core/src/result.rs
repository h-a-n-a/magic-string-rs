use std::result;

#[derive(Debug)]
pub enum MagicStringErrorType {
  VLQEncodingError,
  UTF8EncodingError,
  JSONSerializationError,
}

#[derive(Debug)]
pub struct Error {
  pub error_type: MagicStringErrorType,
  pub reason: Option<String>,
}

impl Error {
  pub fn new(error_type: MagicStringErrorType) -> Self {
    Self {
      error_type,
      reason: None,
    }
  }

  pub fn new_with_reason(error_type: MagicStringErrorType, reason: &str) -> Self {
    Self {
      error_type,
      reason: Some(String::from(reason)),
    }
  }
}

pub type Result<T = ()> = result::Result<T, Error>;
