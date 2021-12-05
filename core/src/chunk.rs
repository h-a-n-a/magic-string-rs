use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
  pub start: usize,
  pub end: usize,
  pub original_str: String,

  pub content: String,

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

  pub fn each_next<F>(chunk: Rc<RefCell<Chunk>>, mut f: F)
  where
    F: FnMut(Rc<RefCell<Chunk>>) -> (),
  {
    let mut curr = Some(chunk);
    while let Some(value) = curr {
      f(Rc::clone(&value));
      curr = {
        if let Some(ref c) = value.borrow().next {
          Some(Rc::clone(c))
        } else {
          None
        }
      };
    }
  }

  pub fn contains(&self, index: usize) -> bool {
    index >= self.start && index < self.end
  }

  pub fn split(chunk: Rc<RefCell<Chunk>>, index: usize) -> Rc<RefCell<Self>> {
    let mut borrowed_chunk = chunk.borrow_mut();
    let chunk_str = borrowed_chunk.original_str[0..(index - borrowed_chunk.start)].to_owned();
    let next_chunk_str = borrowed_chunk.original_str[(index - borrowed_chunk.start)..].to_owned();

    let next_chunk = Rc::new(RefCell::new(Chunk::new(
      index,
      borrowed_chunk.end,
      next_chunk_str.as_str(),
    )));

    borrowed_chunk.original_str = chunk_str.to_owned();
    borrowed_chunk.content = chunk_str.to_owned();
    borrowed_chunk.end = index;

    /* Outro of the current chunk will be moved to the newly created one
     * and we need to reset the current one
     */
    borrowed_chunk.outro = String::from("");

    next_chunk.borrow_mut().outro = borrowed_chunk.outro.to_owned();
    next_chunk.borrow_mut().next = {
      if let Some(_) = borrowed_chunk.next {
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
