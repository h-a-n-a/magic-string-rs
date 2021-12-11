use std::{cell::RefCell, collections::HashMap, rc::Rc, string::ToString};

#[cfg(feature = "node-api")]
use napi_derive::napi;

use crate::{
  chunk::Chunk,
  mapping::{Mapping, Mappings},
  result::Result,
  source_map::SourceMap,
  utils::locator::Locator,
};

#[cfg(feature = "node-api")]
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct GenerateDecodedMapOptions {
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub source: Option<String>,
  pub include_content: bool,
}

#[cfg(not(feature = "node-api"))]
#[derive(Debug, Default, Clone)]
pub struct GenerateDecodedMapOptions {
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub source: Option<String>,
  pub include_content: bool,
}

#[derive(Debug, Serialize)]
pub struct DecodedMap {
  pub file: Option<String>,
  pub sources: Vec<Option<String>>,
  pub source_root: Option<String>,
  pub sources_content: Vec<Option<String>>,
  pub names: Vec<String>,
  pub mappings: Mappings,
}

#[derive(Debug, Clone)]
pub struct MagicString {
  original_str: String,
  original_str_locator: Locator,

  // prefix
  intro: String,
  // suffix
  outro: String,

  chunk_by_start: HashMap<u32, Rc<RefCell<Chunk>>>,
  chunk_by_end: HashMap<u32, Rc<RefCell<Chunk>>>,

  last_searched_chunk: Rc<RefCell<Chunk>>,
  first_chunk: Rc<RefCell<Chunk>>,
  last_chunk: Rc<RefCell<Chunk>>,
}

impl MagicString {
  /// ## Create a new `MagicString` instance
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("import React from 'react'");
  ///
  /// assert_eq!(s.to_string(), "import React from 'react'");
  /// ```
  ///
  ///
  pub fn new(str: &str) -> MagicString {
    let original_chunk = Rc::new(RefCell::new(Chunk::new(0u32, str.len() as u32, str)));

    MagicString {
      original_str: String::from(str),

      // prepends and appends are followed with current instance or chunk
      intro: String::default(),
      outro: String::default(),

      chunk_by_start: HashMap::default(),
      chunk_by_end: HashMap::default(),

      first_chunk: Rc::clone(&original_chunk),
      last_chunk: Rc::clone(&original_chunk),
      last_searched_chunk: Rc::clone(&original_chunk),

      original_str_locator: Locator::new(str),
    }
  }

  /// ## Append `string`
  ///
  /// Appends the specified content to the end of the string. Returns `self`.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("import React from 'react'");
  ///
  /// s.append("\nexport default React");
  ///
  /// assert_eq!(s.to_string(), "import React from 'react'\nexport default React");
  ///
  /// ```
  pub fn append(&mut self, str: &str) -> Result<&mut Self> {
    self.outro = format!("{}{}", self.outro, str);

    Ok(self)
  }

  /// ## Prepend `string`
  ///
  /// Prepends the string with the specified content. Returns `self`.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("export default React");
  ///
  /// s.prepend("import React from 'react'\n");
  ///
  /// assert_eq!(s.to_string(), "import React from 'react'\nexport default React");
  ///
  /// ```
  pub fn prepend(&mut self, str: &str) -> Result<&mut Self> {
    self.intro = format!("{}{}", self.intro, str);

    Ok(self)
  }

  /// ## Prepend left
  ///
  /// Same as `s.append_left(...)`, except that the inserted content will go before any previous appends or prepends at index.
  pub fn prepend_left(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().prepend_outro(str);
    } else {
      self.intro = format!("{}{}", str, self.intro)
    };

    Ok(self)
  }

  /// ## Prepend right
  ///
  /// Same as `s.append_right(...)`, except that the inserted content will go before any previous appends or prepends at index.
  pub fn prepend_right(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().prepend_intro(str);
    } else {
      self.outro = format!("{}{}", str, self.outro)
    };

    Ok(self)
  }

  /// ## Append left
  ///
  /// Appends the specified content at the index in the original string.
  /// If a range ending with index is subsequently moved, the insert will be moved with it. Returns this. See also `s.prepend_left(...)`.
  pub fn append_left(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().append_outro(str);
    } else {
      self.prepend(str)?;
    };

    Ok(self)
  }

  /// ## Append right
  ///
  /// Appends the specified content at the index in the original string.
  /// If a range starting with index is subsequently moved, the insert will be moved with it. Returns this. See also `s.prepend_right(...)`.
  pub fn append_right(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().append_intro(str);
    } else {
      self.append(str)?;
    };

    Ok(self)
  }

  /// ## Generate decoded map
  ///
  /// Generates a sourcemap object with raw mappings in array form, rather than encoded as a string.
  /// See generate_map documentation below for options details.
  /// Useful if you need to manipulate the sourcemap further, but most of the time you will use generateMap instead.
  ///
  /// Notice: All decoded mappings are positioned absolutely.
  ///
  /// Example
  /// ```
  /// use magic_string::{MagicString, GenerateDecodedMapOptions};
  ///
  /// let mut s = MagicString::new("export default React");
  /// s.prepend("import React from 'react'\n");
  ///
  /// s.generate_decoded_map(GenerateDecodedMapOptions {
  ///   file: Some("index.js".to_owned()),
  ///   source: Some("index.ts".to_owned()),
  ///   source_root: Some("./".to_owned()),
  ///   include_content: true
  /// });
  /// ```
  pub fn generate_decoded_map(&mut self, options: GenerateDecodedMapOptions) -> Result<DecodedMap> {
    let mut map = Mapping::new();
    let locator = &self.original_str_locator;

    map.advance(self.intro.as_str());

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let loc = locator.locate(chunk.borrow().start);
      map.add_unedited_chunk(Rc::clone(&chunk), loc);
    });

    map.advance(self.outro.as_str());

    Ok(DecodedMap {
      file: options.file.to_owned(),
      mappings: map.get_decoded_mappings(),
      source_root: options.source_root.to_owned(),
      sources: vec![options.source],
      names: vec![],
      sources_content: {
        if options.include_content {
          vec![Some(self.original_str.to_owned())]
        } else {
          vec![None]
        }
      },
    })
  }

  /// ## Generate Map
  ///
  /// Generates a version 3 sourcemap. All options are optional, see `GenerateDecodedMapOptions` for detailed document.
  ///
  /// ```
  /// use magic_string::{MagicString, GenerateDecodedMapOptions};
  ///
  /// let mut s = MagicString::new("export default React");
  /// s.prepend("import React from 'react'\n");
  ///
  /// let generated_map = s.generate_map(GenerateDecodedMapOptions {
  ///   file: Some("index.js".to_owned()),
  ///   source: Some("index.ts".to_owned()),
  ///   source_root: Some("./".to_owned()),
  ///   include_content: true
  /// }).expect("fail to generate map");
  ///
  /// generated_map.to_string(); // generates v3 sourcemap in JSON format
  /// generated_map.to_url(); // generates v3 inline sourcemap
  /// ```
  pub fn generate_map(&mut self, options: GenerateDecodedMapOptions) -> Result<SourceMap> {
    let decoded_map = self.generate_decoded_map(options)?;
    SourceMap::new_from_decoded(decoded_map)
  }

  fn _split_at_index(&mut self, index: u32) {
    if self.chunk_by_end.contains_key(&index) || self.chunk_by_start.contains_key(&index) {
      // early bail-out if it's already split
      return;
    }

    let chunk = Rc::clone(&self.last_searched_chunk);

    let search_forward = index > chunk.borrow().start;

    let mut curr = Some(&chunk);
    while let Some(c) = curr {
      if c.borrow().contains(index) {
        // FIXME: use static method to satisfy the borrow checker
        self._split_chunk_at_index(Rc::clone(c), index);
        return;
      } else {
        curr = {
          if search_forward {
            self.chunk_by_start.get(&c.borrow().end)
          } else {
            self.chunk_by_end.get(&c.borrow().start)
          }
        }
      }
    }
  }

  fn _split_chunk_at_index(&mut self, chunk: Rc<RefCell<Chunk>>, index: u32) {
    let new_chunk = Chunk::split(Rc::clone(&chunk), index);

    let new_chunk_original = new_chunk.borrow();
    self.chunk_by_end.insert(index, Rc::clone(&chunk));

    self.chunk_by_start.insert(index, Rc::clone(&new_chunk));
    self
      .chunk_by_end
      .insert(new_chunk_original.end, Rc::clone(&new_chunk));

    if self.last_chunk == chunk {
      self.last_chunk = Rc::clone(&new_chunk);
    }

    self.last_searched_chunk = Rc::clone(&chunk);
  }
}

impl ToString for MagicString {
  fn to_string(&self) -> String {
    let mut str = self.intro.to_owned();

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let chunk = chunk.borrow();
      str = format!("{}{}{}{}", str, chunk.intro, chunk.content, chunk.outro);
    });

    format!("{}{}", str, self.outro)
  }
}
