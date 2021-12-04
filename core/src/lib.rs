mod mapping;
mod utils;

mod chunk;
mod source_map;

pub mod magic_string;
pub mod result;
pub use crate::magic_string::MagicString;
pub use crate::result::{Error, MagicStringErrorType, Result};
