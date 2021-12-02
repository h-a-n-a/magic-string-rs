use std::borrow::Borrow;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
  pub start: usize,
  pub end: usize,
  pub original_str: String,

  pub intro: String,
  pub outro: String,

  pub next: Option<Rc<RefCell<Chunk>>>,
  pub prev: Option<Rc<RefCell<Chunk>>>,
}

impl Chunk {
  pub fn new(start: usize, end: usize, content: &str) -> Chunk {
    Chunk {
      start,
      end,
      original_str: String::from(content),

      intro: String::from(""),
      outro: String::from(""),

      next: None,
      prev: None,
    }
  }

  pub fn split(&mut self, index: usize) -> Result<Self, ()> {
    let chunk_str = self.original_str[0..index].to_owned();
    let next_chunk_str = self.original_str[index..].to_owned();

    let mut next_chunk = Chunk::new(index, self.end, next_chunk_str);

    self.original_str = chunk_str;
    self.end = index;

    /* Outro of the current chunk will be moved to the newly created one
     * and we need to reset the current one
     */
    next_chunk.outro = self.outro.to_owned();
    self.outro = String::from("");

    next_chunk.next = {
      if let Some(_) = self.next {
        self.next.clone()
      } else {
        None
      }
    };

    Ok(next_chunk)
  }

  pub fn each_next<F>(&self, f: F)
  where
    F: Fn(Rc<RefCell<Chunk>>) -> (),
  {
    let mut curr = Some(Rc::new(RefCell::new((*self).clone())));
    while let Some(ref value) = curr {
      f(Rc::clone(value));
      curr = curr.next;
    }
  }

  pub fn contains(&self, index: usize) -> bool {
    index >= self.start && index < self.end
  }
}

// impl Iterator for Chunk {
//   type Item = Rc<RefCell<Self>>;
//
//   fn next(&mut self) -> Option<Self::Item> {
//     if let Some(ref next) = self.next {
//       Some(Rc::clone(next))
//     } else {
//       None
//     }
//   }
// }
