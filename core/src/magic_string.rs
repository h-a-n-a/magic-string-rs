use std::{cell::RefCell, collections::HashMap, rc::Rc, string::ToString};

use crate::utils::trim;

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

#[cfg(feature = "node-api")]
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct OverwriteOptions {
  pub store_name: bool,
  pub content_only: bool,
}

#[cfg(not(feature = "node-api"))]
#[derive(Debug, Default, Clone)]
pub struct OverwriteOptions {
  pub store_name: bool,
  pub content_only: bool,
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

  intro: String,
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
  /// Same as `s.append_left(...)`, except that the inserted content will go before any previous appends or prepends at index. Returns `self`.
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
  /// Same as `s.append_right(...)`, except that the inserted content will go before any previous appends or prepends at index. Returns `self`.
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
  /// If a range ending with index is subsequently moved, the insert will be moved with it. Returns this. See also `s.prepend_left(...)`. Returns `self`.
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
  /// If a range starting with index is subsequently moved, the insert will be moved with it. Returns this. See also `s.prepend_right(...)`. Returns `self`.
  pub fn append_right(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().append_intro(str);
    } else {
      self.append(str)?;
    };

    Ok(self)
  }

  /// ## Overwrite
  ///
  /// Replaces the characters from start to end with content. The same restrictions as s.remove() apply. Returns `self`.
  /// The fourth argument is optional.
  /// - It can have a `store_name` property — if true, the original name will be stored for later inclusion in a sourcemap's names array
  /// - and a `content_only` property which determines whether only the content is overwritten, or anything that was appended/prepended to the range as well.
  pub fn overwrite(
    &mut self,
    start: u32,
    end: u32,
    content: &str,
    options: Option<OverwriteOptions>,
  ) -> Result<&mut Self> {
    todo!()
  }

  /// ## Trim start and end
  ///
  /// Trims content matching `pattern` (defaults to '\s', i.e. whitespace) from the start and the end. Returns `self`.
  /// Note that in Rust, '\t'(char) and "\\t"(string) are different types, whereas they are regarded the same pattern in Regex, which means you can pass eiter one of them to `pattern` argument.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("  abc  ");
  /// s.trim(None);
  ///
  /// assert_eq!(s.to_string(), "abc");
  ///
  /// let mut s = MagicString::new("\t\t abc \t\t");
  /// s.trim(Some("\t"));
  ///
  /// assert_eq!(s.to_string(), " abc ");
  ///
  /// let mut s = MagicString::new("\t\t abc \t\t");
  /// s.trim(Some("\t|\\s"));
  ///
  /// assert_eq!(s.to_string(), "abc");
  /// ```
  pub fn trim(&mut self, pattern: Option<&str>) -> Result<&mut Self> {
    self.trim_start(pattern)?.trim_end(pattern)
  }

  /// ## Trim start
  ///
  /// Trims content matching `pattern` (defaults to '\s', i.e. whitespace) from the start. Returns `self`.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("  abc");
  /// s.trim_start(None);
  ///
  /// assert_eq!(s.to_string(), "abc");
  ///
  /// let mut s = MagicString::new("  abc");
  /// s.prepend("  ");
  /// s.trim_start(None);
  ///
  /// assert_eq!(s.to_string(), "abc");
  ///
  /// let mut s = MagicString::new("  abc");
  /// s.prepend("\t\ta");
  /// s.trim_start(Some("\t"));
  ///
  /// assert_eq!(s.to_string(), "a  abc");
  /// ```
  pub fn trim_start(&mut self, pattern: Option<&str>) -> Result<&mut Self> {
    let pattern = pattern.unwrap_or("\\s");

    self.intro = trim::trim_start_regexp(self.intro.as_str(), pattern)?.to_owned();

    if self.intro.len() > 0 {
      return Ok(self);
    }

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      self.last_searched_chunk = Rc::clone(&chunk);
      chunk
        .borrow_mut()
        .trim_start_regexp(pattern)
        .expect("[magic-string] Regexp Error");

      !(chunk.borrow_mut().to_string().len() > 0)
    });

    if self.last_searched_chunk == self.last_chunk
      && (self.last_chunk.borrow().to_string().len() == 0)
    {
      self.outro = trim::trim_start_regexp(self.outro.as_str(), pattern)?.to_owned()
    }

    Ok(self)
  }

  /// ## Trim end
  ///
  /// Trims content matching `pattern` (defaults to '\s', i.e. whitespace) from the end. Returns `self`.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("abc  ");
  /// s.trim_end(None);
  ///
  /// assert_eq!(s.to_string(), "abc");
  ///
  /// let mut s = MagicString::new("abc  ");
  /// s.append("  ");
  /// s.trim_end(None);
  ///
  /// assert_eq!(s.to_string(), "abc");
  ///
  /// let mut s = MagicString::new("abc");
  /// s.append("  a\t\t");
  /// s.trim_end(Some("\t"));
  ///
  /// assert_eq!(s.to_string(), "abc  a");
  pub fn trim_end(&mut self, pattern: Option<&str>) -> Result<&mut Self> {
    let pattern = pattern.unwrap_or("\\s");

    self.outro = trim::trim_end_regexp(self.outro.as_str(), pattern)?.to_owned();

    if self.outro.len() > 0 {
      return Ok(self);
    }

    Chunk::each_prev(Rc::clone(&self.last_chunk), |chunk| {
      self.last_searched_chunk = Rc::clone(&chunk);
      chunk
        .borrow_mut()
        .trim_end_regexp(pattern)
        .expect("[magic-string] Regexp Error");

      !(chunk.borrow_mut().to_string().len() > 0)
    });

    if self.last_searched_chunk == self.first_chunk
      && (self.first_chunk.borrow().to_string().len() == 0)
    {
      self.intro = trim::trim_end_regexp(self.intro.as_str(), pattern)?.to_owned()
    }

    Ok(self)
  }

  /// ## Trim lines
  ///
  /// Removes empty lines from the start and end. Returns `self`.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("\n\nabc\n");
  /// s.append("\n");
  /// s.prepend("\n");
  ///
  /// s.trim_lines();
  ///
  /// assert_eq!(s.to_string(), "abc")
  /// ```
  pub fn trim_lines(&mut self) -> Result<&mut Self> {
    self.trim_start(Some("\n"))?.trim_end(Some("\n"))
  }

  /// ## Is empty
  ///
  /// Returns `true` if the resulting source is empty (disregarding white space).
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("");
  ///
  /// assert_eq!(s.is_empty(), true);
  ///
  /// let mut s = MagicString::new("abc");
  ///
  /// assert_eq!(s.is_empty(), false);
  /// ```
  pub fn is_empty(&mut self) -> bool {
    self.to_string().trim().is_empty()
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
      map.add_chunk(Rc::clone(&chunk), loc);
      true
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
  /// ## To string
  ///
  /// Returns a modified string.
  ///
  /// Example:
  /// ```
  /// use magic_string::MagicString;
  ///
  /// let mut s = MagicString::new("abc");
  ///
  /// assert_eq!(s.to_string(), "abc");
  /// ```
  fn to_string(&self) -> String {
    let mut str = self.intro.to_owned();

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let chunk = chunk.borrow();
      str = format!("{}{}{}{}", str, chunk.intro, chunk.content, chunk.outro);
      true
    });

    format!("{}{}", str, self.outro)
  }
}
