use std::{cell::RefCell, collections::HashMap, rc::Rc, result, string::ToString};

use crate::{chunk::Chunk, mapping::Mapping, source_map::SourceMap, utils::locator::Locator};

#[derive(Debug)]
pub enum MagicStringErrorType {
  VLQEncodingError,
  UTF8EncodingError,
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

#[derive(Debug)]
pub struct MagicString {
  pub(crate) original_str: String,
  pub(crate) original_str_locator: Locator,

  // prefix
  pub(crate) intro: String,
  // suffix
  pub(crate) outro: String,

  chunk_by_start: HashMap<usize, Rc<RefCell<Chunk>>>,
  chunk_by_end: HashMap<usize, Rc<RefCell<Chunk>>>,

  last_searched_chunk: Rc<RefCell<Chunk>>,
  pub first_chunk: Rc<RefCell<Chunk>>,
  last_chunk: Rc<RefCell<Chunk>>,
}

impl MagicString {
  pub fn new(str: &str) -> MagicString {
    let original_chunk = Rc::new(RefCell::new(Chunk::new(0usize, str.len(), str)));

    MagicString {
      original_str: String::from(str),

      // prepends and appends are followed with current instance or chunk
      intro: String::from(""),
      outro: String::from(""),

      chunk_by_start: HashMap::new(),
      chunk_by_end: HashMap::new(),

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

  pub fn prepend_left(&mut self, index: usize, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().prepend_outro(str);
    } else {
      self.intro = format!("{}{}", str, self.intro)
    };

    Ok(self)
  }

  pub fn prepend_right(&mut self, index: usize, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().prepend_intro(str);
    } else {
      self.outro = format!("{}{}", str, self.outro)
    };

    Ok(self)
  }

  pub fn append_left(&mut self, index: usize, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_end.get(&index) {
      chunk.borrow_mut().append_outro(str);
    } else {
      self.prepend(str)?;
    };

    Ok(self)
  }

  pub fn append_right(&mut self, index: usize, str: &str) -> Result<&mut Self> {
    self._split_at_index(index);

    if let Some(chunk) = self.chunk_by_start.get(&index) {
      chunk.borrow_mut().append_intro(str);
    } else {
      self.append(str)?;
    };

    Ok(self)
  }

  pub fn generate_decoded_map(&mut self) -> Result<Mapping> {
    let mut map = Mapping::new();
    let locator = &self.original_str_locator;

    map.advance(self.intro.as_str());

    let first_chunk = self.first_chunk.borrow();

    first_chunk.each_next(|chunk| {
      let loc = locator.locate(chunk.borrow().start);
      map.add_unedited_chunk(Rc::clone(&chunk), loc);
    });

    map.advance(self.outro.as_str());

    Ok(map)
  }

  pub fn generate_map(&mut self) -> Result<SourceMap> {
    Ok(SourceMap::new(
      self
        .generate_decoded_map()?
        .get_encoded_mappings()?
        .as_str(),
    ))
  }

  fn _split_at_index(&mut self, index: usize) {
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

  fn _split_chunk_at_index(&mut self, chunk: Rc<RefCell<Chunk>>, index: usize) {
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
    let mut str = String::from(self.intro.to_owned());

    self.first_chunk.borrow().each_next(|chunk| {
      let chunk = chunk.borrow();
      str = format!("{}{}{}{}", str, chunk.intro, chunk.content, chunk.outro);
    });

    format!("{}{}", str, self.outro)
  }
}
