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

  pub fn append(&mut self, str: &str) -> Result<&mut Self> {
    self.outro = format!("{}{}", self.outro, str);

    Ok(self)
  }

  pub fn prepend(&mut self, str: &str) -> Result<&mut Self> {
    self.intro = format!("{}{}", self.intro, str);

    Ok(self)
  }

  pub fn prepend_left(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().prepend_outro(str);
    } else {
      self.intro = format!("{}{}", str, self.intro)
    };

    Ok(self)
  }

  pub fn prepend_right(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().prepend_intro(str);
    } else {
      self.outro = format!("{}{}", str, self.outro)
    };

    Ok(self)
  }

  pub fn append_left(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().append_outro(str);
    } else {
      self.prepend(str)?;
    };

    Ok(self)
  }

  pub fn append_right(&mut self, index: u32, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().append_intro(str);
    } else {
      self.append(str)?;
    };

    Ok(self)
  }

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
