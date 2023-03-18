use base64::Engine;
use serde::Serialize;

use crate::magic_string::DecodedMap;
use crate::mapping::Mapping;
use crate::result::Result;

// current specification version
static VERSION: u8 = 3;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
  pub version: u8,
  pub mappings: String,
  pub names: Vec<String>,
  pub sources: Vec<Option<String>>,
  pub sources_content: Vec<Option<String>>,
  pub file: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  pub source_root: Option<String>,
}

impl SourceMap {
  pub fn new(
    mappings: &str,
    file: Option<&str>,
    names: Vec<&str>,
    sources_content: Vec<Option<&str>>,
    source_root: Option<&str>,
    sources: Vec<Option<&str>>,
  ) -> Self {
    Self {
      version: VERSION,
      mappings: String::from(mappings),
      file: file.map(|f| f.to_owned()),
      names: names.iter().map(|&n| n.to_owned()).collect::<Vec<String>>(),
      sources_content: sources_content
        .iter()
        .map(|s| s.map(|s| s.to_owned()))
        .collect(),
      source_root: source_root.map(|s| s.to_owned()),
      sources: sources.iter().map(|s| s.map(|s| s.to_owned())).collect(),
    }
  }

  /// ## Create a SourceMap instance from a decoded map
  ///
  /// `DecodedMap` can be created by utilizing `generate_decoded_map`.
  ///
  /// Example:
  /// ```
  /// use magic_string::{MagicString, GenerateDecodedMapOptions, SourceMap};
  ///
  /// let mut s = MagicString::new("export default React");
  /// s.prepend("import React from 'react'\n");
  ///
  /// let decoded_map = s.generate_decoded_map(GenerateDecodedMapOptions {
  /// file: Some("index.js".to_owned()),
  ///   source: Some("index.ts".to_owned()),
  ///   source_root: Some("./".to_owned()),
  ///   include_content: true,
  ///   hires: false,
  /// }).expect("failed to generate decoded map");
  ///
  /// SourceMap::new_from_decoded(decoded_map);
  /// ```
  pub fn new_from_decoded(decoded_map: DecodedMap) -> Result<Self> {
    Ok(Self {
      version: VERSION,
      file: decoded_map.file,
      mappings: Mapping::generate_encoded_mappings(&decoded_map.mappings)?,
      names: decoded_map.names,
      source_root: decoded_map.source_root,
      sources_content: decoded_map.sources_content,
      sources: decoded_map.sources,
    })
  }

  /// ## Generate SourceMap in JSON format
  pub fn to_string(&self) -> Result<String> {
    Ok(serde_json::to_string(self)?)
  }

  /// ## Generate inline SourceMap
  pub fn to_url(&self) -> Result<String> {
    let str = Self::to_string(self)?;

    Ok(format!(
      "data:application/json;charset=utf-8;base64,{}",
      base64::engine::general_purpose::STANDARD.encode(str)
    ))
  }
}
