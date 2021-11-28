use std::{cell::RefCell, collections::HashMap, rc::Rc, string::ToString};

use crate::chunk::Chunk;

#[derive(Debug)]
pub struct MagicString {
  pub original_str: String,

  /* prefix */
  pub intro: String,
  /* suffix */
  pub outro: String,

  chunk_by_start: HashMap<usize, Rc<RefCell<Chunk>>>,
  chunk_by_end: HashMap<usize, Rc<RefCell<Chunk>>>,

  last_searched_chunk: Rc<RefCell<Chunk>>,
  first_chunk: Rc<RefCell<Chunk>>,
  last_chunk: Rc<RefCell<Chunk>>,
}

impl MagicString {
  pub fn new(str: &str) -> MagicString {
    let original_chunk = Rc::new(RefCell::new(Chunk::new(
      0usize,
      str.len(),
      String::from(str),
    )));

    MagicString {
      original_str: String::from(str),

      intro: String::from(""),
      outro: String::from(""),

      chunk_by_start: HashMap::new(),
      chunk_by_end: HashMap::new(),

      first_chunk: Rc::clone(&original_chunk),
      last_chunk: Rc::clone(&original_chunk),
      last_searched_chunk: Rc::clone(&original_chunk),
    }
  }

  pub fn append(&mut self, str: &str) -> Result<&mut Self, ()> {
    self.outro = format!("{}{}", self.outro, str);

    Ok(self)
  }

  pub fn prepend(&mut self, str: &str) -> Result<&mut Self, ()> {
    self.intro = format!("{}{}", self.intro, str);

    Ok(self)
  }

  pub fn prepend_left(&mut self, index: usize, str: &str) -> Result<&mut Self, ()> {
    todo!()
  }

  pub fn prepend_right(&mut self, index: usize, str: &str) -> Result<&mut Self, ()> {
    todo!()
  }

  pub fn append_left(&mut self, index: usize, str: &str) -> Result<&mut Self, ()> {
    todo!()
  }

  pub fn append_right(&mut self, index: usize, str: &str) -> Result<&mut Self, ()> {
    todo!()
  }

  pub fn generate_decoded_map(&mut self) -> Result<&mut Self, ()> {
    todo!()
  }

  pub fn generate_map(&mut self) -> Result<&mut Self, ()> {
    todo!()
  }

  fn _split(&mut self, index: usize) -> Result<(), ()> {
    if self.chunk_by_end.contains_key(&index) || self.chunk_by_start.contains_key(&index) {
      Ok(())
    } else {
      let chunk = Rc::clone(&self.last_searched_chunk);

      let search_forward = index > chunk.borrow().start;

      let mut curr = Some(&chunk);
      while let Some(c) = curr {
        if c.borrow().contains(index) {
          return self._split_chunk(Rc::clone(c), index);
        }

        curr = {
          if search_forward {
            self.chunk_by_end.get(&index)
          } else {
            self.chunk_by_start.get(&index)
          }
        };
      }

      Ok(())
    }
  }

  fn _split_chunk(&mut self, chunk: Rc<RefCell<Chunk>>, index: usize) -> Result<(), ()> {
    let new_chunk = Rc::new(RefCell::new(chunk.borrow_mut().split(index)?));

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

    Ok(())
  }
}

impl ToString for MagicString {
  fn to_string(&self) -> String {
    // FIXME: replace original_str to chunks
    format!("{}{}{}", self.intro, self.original_str, self.outro)
  }
}
