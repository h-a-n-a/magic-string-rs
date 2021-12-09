use std::fmt::Formatter;
use std::{
  fmt, io, result,
  string::{self, FromUtf8Error},
};

#[derive(Debug, Clone)]
pub enum MagicStringErrorType {
  IOError,
  UTF8Error,

  JSONSerializationError,

  VlqUnexpectedEof,
  VlqInvalidBase64,
  VlqOverflow,
}

pub type Result<T = ()> = result::Result<T, Error>;

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

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(ref reason) = self.reason {
      write!(f, "{:?}, {}", self.error_type, reason)
    } else {
      write!(f, "{:?}", self.error_type)
    }
  }
}

impl From<io::Error> for Error {
  #[inline]
  fn from(_: io::Error) -> Self {
    Error::new(MagicStringErrorType::IOError)
  }
}

impl From<vlq::Error> for Error {
  #[inline]
  fn from(err: vlq::Error) -> Self {
    match err {
      vlq::Error::UnexpectedEof => Error::new(MagicStringErrorType::VlqUnexpectedEof),
      vlq::Error::InvalidBase64(_) => Error::new(MagicStringErrorType::VlqInvalidBase64),
      vlq::Error::Overflow => Error::new(MagicStringErrorType::VlqOverflow),
    }
  }
}

impl From<string::FromUtf8Error> for Error {
  #[inline]
  fn from(_: FromUtf8Error) -> Self {
    Error::new(MagicStringErrorType::UTF8Error)
  }
}

impl From<serde_json::Error> for Error {
  #[inline]
  fn from(_: serde_json::Error) -> Self {
    Error::new(MagicStringErrorType::JSONSerializationError)
  }
}

#[cfg(feature = "node-api")]
impl From<Error> for napi::Error {
  #[inline]
  fn from(err: Error) -> Self {
    let mut reason = String::from("[magic-string] ");

    match err.error_type {
      MagicStringErrorType::VlqUnexpectedEof => {
        reason.push_str("Vlq Unexpected Eof");
      }
      MagicStringErrorType::VlqInvalidBase64 => {
        reason.push_str("Vlq Unexpected Base64");
      }
      MagicStringErrorType::VlqOverflow => {
        reason.push_str("Vlq Overflow");
      }
      MagicStringErrorType::IOError => {
        reason.push_str("IO Error");
      }
      MagicStringErrorType::UTF8Error => {
        reason.push_str("UTF8 Encoding Error");
      }
      MagicStringErrorType::JSONSerializationError => {
        reason.push_str("JSON Serialization Error");
      }
    }

    if let Some(r) = err.reason {
      reason.push_str(", ");
      reason.push_str(&r[..]);
    }

    napi::Error::new(napi::Status::GenericFailure, reason)
  }
}
