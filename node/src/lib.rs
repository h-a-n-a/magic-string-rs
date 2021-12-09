extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::Result;

#[napi]
pub struct MagicString(magic_string::MagicString);

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
  pub fn to_json_sourcemap(
    &mut self,
    options: Option<magic_string::GenerateDecodedMapOptions>,
  ) -> Result<String> {
    Ok(
      self
        .0
        .generate_map(options.unwrap_or_default())?
        .to_json()?,
    )
  }

  #[napi]
  pub fn to_url_sourcemap(
    &mut self,
    options: Option<magic_string::GenerateDecodedMapOptions>,
  ) -> Result<String> {
    Ok(self.0.generate_map(options.unwrap_or_default())?.to_url()?)
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
