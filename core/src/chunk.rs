use std::{cell::RefCell, rc::Rc};

use crate::utils::trim;
use crate::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
  pub start: u32,
  pub end: u32,
  pub original_str: String,

  pub content: String,

  pub intro: String,
  pub outro: String,

  pub next: Option<Rc<RefCell<Chunk>>>,
  pub prev: Option<Rc<RefCell<Chunk>>>,
}

impl Chunk {
  pub fn new(start: u32, end: u32, content: &str) -> Chunk {
    Chunk {
      start,
      end,
      original_str: String::from(content),
      content: String::from(content),

      intro: String::default(),
      outro: String::default(),

      next: None,
      prev: None,
    }
  }

  // The original `MagicString`'s naming looks a little bit weird to me
  // So I have to change this, please forgive me...

  pub fn append_outro(&mut self, content: &str) {
    self.outro = format!("{}{}", self.outro, content);
  }

  pub fn prepend_outro(&mut self, content: &str) {
    self.outro = format!("{}{}", content, self.outro);
  }

  pub fn append_intro(&mut self, content: &str) {
    self.intro = format!("{}{}", self.intro, content);
  }

  pub fn prepend_intro(&mut self, content: &str) {
    self.intro = format!("{}{}", content, self.intro);
  }

  pub fn trim_start_regexp(&mut self, pat: &str) -> Result {
    let trimmed_intro = trim::trim_start_regexp(self.intro.as_str(), pat)?.to_owned();
    self.intro = trimmed_intro.to_owned();
    if !trimmed_intro.is_empty() {
      return Ok(());
    }

    let trimmed_content = trim::trim_start_regexp(self.content.as_str(), pat)?.to_owned();
    self.content = trimmed_content.to_owned();
    if !trimmed_content.is_empty() {
      return Ok(());
    }

    let trimmed_outro = trim::trim_start_regexp(self.outro.as_str(), pat)?.to_owned();
    self.outro = trimmed_outro;

    Ok(())
  }

  pub fn trim_end_regexp(&mut self, pat: &str) -> Result {
    let trimmed_outro = trim::trim_end_regexp(self.outro.as_str(), pat)?.to_owned();
    self.outro = trimmed_outro.to_owned();

    if !trimmed_outro.is_empty() {
      return Ok(());
    }

    let trimmed_content = trim::trim_end_regexp(self.content.as_str(), pat)?.to_owned();
    self.content = trimmed_content.to_owned();
    if !trimmed_content.is_empty() {
      return Ok(());
    }

    let trimmed_intro = trim::trim_end_regexp(self.intro.as_str(), pat)?.to_owned();
    self.intro = trimmed_intro;

    Ok(())
  }

  pub fn is_content_edited(&self) -> bool {
    self.original_str.len() != self.content.len() || self.original_str != self.content
  }

  pub fn is_edited(&self) -> bool {
    self.is_content_edited() || !self.intro.is_empty() || !self.outro.is_empty()
  }

  pub fn try_each_next<F>(chunk: Rc<RefCell<Chunk>>, mut f: F) -> Result
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> Result<bool>,
  {
    let mut curr = Some(chunk);
    while let Some(value) = curr {
      match f(Rc::clone(&value)) {
        Ok(should_yield) => {
          if should_yield {
            break;
          }
        }
        Err(e) => {
          return Err(e);
        }
      }
      curr = value.borrow().next.as_ref().map(Rc::clone);
    }
    Ok(())
  }

  pub fn try_each_prev<F>(chunk: Rc<RefCell<Chunk>>, mut f: F) -> Result
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> Result<bool>,
  {
    let mut curr = Some(chunk);
    while let Some(value) = curr {
      match f(Rc::clone(&value)) {
        Ok(should_yield) => {
          if should_yield {
            break;
          }
        }
        Err(e) => return Err(e),
      }
      curr = value.borrow().prev.as_ref().map(Rc::clone);
    }
    Ok(())
  }

  pub fn contains(&self, index: u32) -> bool {
    index >= self.start && index < self.end
  }

  pub fn split(chunk: Rc<RefCell<Chunk>>, index: u32) -> Rc<RefCell<Self>> {
    let mut curr_chunk = chunk.borrow_mut();

    let chunk_mid = (index - curr_chunk.start) as usize;
    let chunk_str = curr_chunk.original_str[0..chunk_mid].to_owned();
    let next_chunk_str = curr_chunk.original_str[chunk_mid..].to_owned();

    let next_chunk = Rc::new(RefCell::new(Chunk::new(
      index,
      curr_chunk.end,
      next_chunk_str.as_str(),
    )));

    // `outro` of the current chunk will be moved to the newly created one and we need to reset the current one
    next_chunk.borrow_mut().outro = curr_chunk.outro.to_owned();
    curr_chunk.outro = String::default();

    if curr_chunk.is_content_edited() {
      next_chunk.borrow_mut().content = String::default();
      curr_chunk.content = String::default();
    } else {
      curr_chunk.content = chunk_str.to_owned();
    }

    curr_chunk.original_str = chunk_str.to_owned();
    curr_chunk.end = index;

    next_chunk.borrow_mut().next = {
      if curr_chunk.next.is_some() {
        Some(Rc::clone(curr_chunk.next.as_ref().unwrap()))
      } else {
        None
      }
    };

    curr_chunk.next = Some(Rc::clone(&next_chunk));

    next_chunk.borrow_mut().prev = Some(Rc::clone(&chunk));

    next_chunk
  }
}

impl ToString for Chunk {
  fn to_string(&self) -> String {
    format!("{}{}{}", self.intro, self.content, self.outro)
  }
}
