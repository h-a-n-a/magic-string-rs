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

      intro: String::from(""),
      outro: String::from(""),

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
    if trimmed_intro.len() > 0 {
      return Ok(());
    }

    let trimmed_content = trim::trim_start_regexp(self.content.as_str(), pat)?.to_owned();
    self.content = trimmed_content.to_owned();
    if trimmed_content.len() > 0 {
      return Ok(());
    }

    let trimmed_outro = trim::trim_start_regexp(self.outro.as_str(), pat)?.to_owned();
    self.outro = trimmed_outro.to_owned();

    Ok(())
  }

  pub fn trim_end_regexp(&mut self, pat: &str) -> Result {
    let trimmed_outro = trim::trim_end_regexp(self.outro.as_str(), pat)?.to_owned();
    self.intro = trimmed_outro.to_owned();

    if trimmed_outro.len() > 0 {
      return Ok(());
    }

    let trimmed_content = trim::trim_end_regexp(self.content.as_str(), pat)?.to_owned();
    self.content = trimmed_content.to_owned();
    if trimmed_content.len() > 0 {
      return Ok(());
    }

    let trimmed_intro = trim::trim_end_regexp(self.outro.as_str(), pat)?.to_owned();
    self.intro = trimmed_intro.to_owned();

    Ok(())
  }

  pub fn each_next<F>(chunk: Rc<RefCell<Chunk>>, mut f: F)
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> bool,
  {
    let mut curr = Some(chunk);
    while let Some(value) = curr {
      let should_yield = f(Rc::clone(&value));
      if should_yield {
        break;
      }
      curr = value.borrow().next.as_ref().map(Rc::clone);
    }
  }

  pub fn each_prev<F>(chunk: Rc<RefCell<Chunk>>, mut f: F)
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> bool,
  {
    let mut curr = Some(chunk);
    while let Some(value) = curr {
      let should_yield = f(Rc::clone(&value));
      if should_yield {
        break;
      }
      curr = value.borrow().prev.as_ref().map(Rc::clone);
    }
  }

  pub fn contains(&self, index: u32) -> bool {
    index >= self.start && index < self.end
  }

  pub fn split(chunk: Rc<RefCell<Chunk>>, index: u32) -> Rc<RefCell<Self>> {
    let mut borrowed_chunk = chunk.borrow_mut();
    let chunk_str =
      borrowed_chunk.original_str[0..(index - borrowed_chunk.start) as usize].to_owned();
    let next_chunk_str =
      borrowed_chunk.original_str[(index - borrowed_chunk.start) as usize..].to_owned();

    let next_chunk = Rc::new(RefCell::new(Chunk::new(
      index,
      borrowed_chunk.end,
      next_chunk_str.as_str(),
    )));

    borrowed_chunk.original_str = chunk_str.to_owned();
    borrowed_chunk.content = chunk_str;
    borrowed_chunk.end = index;

    /* Outro of the current chunk will be moved to the newly created one
     * and we need to reset the current one
     */
    borrowed_chunk.outro = String::default();

    next_chunk.borrow_mut().outro = borrowed_chunk.outro.to_owned();
    next_chunk.borrow_mut().next = {
      if borrowed_chunk.next.is_some() {
        Some(Rc::clone(borrowed_chunk.next.as_ref().unwrap()))
      } else {
        None
      }
    };

    borrowed_chunk.next = Some(Rc::clone(&next_chunk));

    next_chunk.borrow_mut().prev = Some(Rc::clone(&chunk));

    next_chunk
  }
}

impl ToString for Chunk {
  fn to_string(&self) -> String {
    format!("{}{}{}", self.intro, self.content, self.outro)
  }
}
