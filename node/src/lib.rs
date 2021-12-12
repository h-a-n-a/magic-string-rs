extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::Result;

use magic_string::SourceMap;

#[napi]
pub struct MagicString(magic_string::MagicString);

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi]
impl MagicString {
  #[napi(constructor)]
  pub fn new(original_str: String) -> Self {
    MagicString(magic_string::MagicString::new(original_str.as_str()))
  }

  #[napi]
  pub fn append(&mut self, input: String) -> Result<&Self> {
    self.0.append(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend(&mut self, input: String) -> Result<&Self> {
    self.0.prepend(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn overwrite(
    &mut self,
    start: i64,
    end: i64,
    content: String,
    options: magic_string::OverwriteOptions,
  ) -> Result<&Self> {
    self.0.overwrite(start, end, content.as_str(), options)?;
    Ok(self)
  }

  #[napi]
  pub fn trim(&mut self, pattern: Option<String>) -> Result<&Self> {
    self.0.trim(pattern.as_ref().map(|p| p.as_str()))?;
    Ok(self)
  }

  #[napi]
  pub fn trim_start(&mut self, pattern: Option<String>) -> Result<&Self> {
    self.0.trim_start(pattern.as_ref().map(|p| p.as_str()))?;
    Ok(self)
  }

  #[napi]
  pub fn trim_end(&mut self, pattern: Option<String>) -> Result<&Self> {
    self.0.trim_end(pattern.as_ref().map(|p| p.as_str()))?;
    Ok(self)
  }

  #[napi]
  pub fn trim_lines(&mut self) -> Result<&Self> {
    self.0.trim_lines()?;
    Ok(self)
  }

  #[napi(ts_return_type = "{ toString: () => string, toUrl: () => string }")]
  pub fn generate_map(
    &mut self,
    options: Option<magic_string::GenerateDecodedMapOptions>,
  ) -> Result<External<SourceMap>> {
    let external = create_external(self.0.generate_map(options.unwrap_or_default())?);
    Ok(external)
  }

  /// @internal
  #[napi]
  pub fn to_sourcemap_string(&mut self, sourcemap: External<SourceMap>) -> Result<String> {
    Ok((*sourcemap.as_ref()).to_string()?)
  }

  /// @internal
  #[napi]
  pub fn to_sourcemap_url(&mut self, sourcemap: External<SourceMap>) -> Result<String> {
    Ok((*sourcemap.as_ref()).to_url()?)
  }

  #[napi(ts_return_type = "DecodedMap")]
  pub fn generate_decoded_map(
    &mut self,
    options: Option<magic_string::GenerateDecodedMapOptions>,
  ) -> Result<String> {
    let decoded = self.0.generate_decoded_map(options.unwrap_or_default())?;
    Ok(serde_json::to_string(&decoded)?)
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }

  #[napi]
  pub fn length(&self) -> u32 {
    self.0.len() as u32
  }
}

#[napi(object)]
/// Only for .d.ts type generation
pub struct DecodedMap {
  pub file: Option<String>,
  pub sources: Vec<Option<String>>,
  pub source_root: Option<String>,
  pub sources_content: Vec<Option<String>>,
  pub names: Vec<String>,
  pub mappings: Vec<Vec<Vec<i64>>>,
}

/// Only for .d.ts generation
#[napi(object)]
pub struct GenerateDecodedMapOptions {
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub source: Option<String>,
  pub include_content: bool,
}
/// Only for .d.ts generation
#[napi(object)]
pub struct OverwriteOptions {
  pub content_only: bool,
}
