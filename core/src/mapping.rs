use std::{cell::RefCell, rc::Rc};

use vlq;

use crate::chunk::Chunk;
use crate::result::{Error, MagicStringErrorType, Result};

static SOURCE_INDEX: u8 = 0;

pub type Segment = Vec<i64>;
pub type Line = Vec<Segment>;
pub type Mappings = Vec<Line>;

#[derive(Debug, PartialEq, Eq)]
pub struct Mapping {
  pub(crate) generated_code_line: usize,
  pub(crate) generated_code_column: usize,

  pub(crate) absolute_mappings: Mappings,
}

impl Mapping {
  pub fn new() -> Self {
    Self {
      generated_code_line: 0,
      generated_code_column: 0,

      // all lines and columns are absolutely related
      // , which is a middle-island for us to convert it to relative mapping later (sourcemap specification)
      absolute_mappings: vec![],
    }
  }

  pub fn add_unedited_chunk(
    &mut self,
    chunk: Rc<RefCell<Chunk>>,
    (original_line, original_column): (usize, usize),
  ) {
    let mut original_line = original_line as i64;
    let original_column = original_column as i64;
    self.advance(chunk.borrow().intro.as_str());

    let original_str = chunk.borrow().original_str.to_owned();
    let original_lines = original_str.split("\n").collect::<Vec<_>>();

    for (index, s) in original_lines.iter().enumerate() {
      if s.is_empty() {
        break;
      }

      let segment: Vec<i64> = vec![
        self.generated_code_column as i64,
        SOURCE_INDEX as i64,
        original_line,
        original_column,
      ];

      if let Some(line) = self.absolute_mappings.get_mut(self.generated_code_line) {
        line.push(segment)
      } else {
        self.absolute_mappings.push(vec![segment])
      }

      if index != original_lines.len() - 1 {
        // We are not at the ending yet, so we have to reset all stuff for new generated lines

        original_line += 1;
        self.generated_code_line += 1;
        self.generated_code_column = 0;
      } else {
        // We are currently at the last piece, this is the next starting piece.
        // So we have to set the next starting column for later use.
        self.generated_code_column += s.len();
      }
    }

    self.advance(chunk.borrow().outro.as_str());
  }

  pub fn advance(&mut self, str: &str) {
    if str.is_empty() {
      return;
    }

    let lines = str.split("\n").collect::<Vec<_>>();

    if lines.len() > 1 {
      self.generated_code_column = 0;
    }

    self.generated_code_line += lines.len() - 1;

    // save starting column for later use
    self.generated_code_column += lines.last().unwrap().len();
  }

  // absolute to relative
  pub fn get_decoded_mappings(&mut self) -> Mappings {
    let mut source_index: i64 = 0;
    let mut original_line: i64 = 0;
    let mut original_column: i64 = 0;

    let decoded_mappings = self
      .absolute_mappings
      .iter()
      .map(|line| {
        let mut generated_column: i64 = 0;

        line
          .iter()
          .map(|segment| {
            let generated_column_offset = segment[0] - generated_column;
            let source_index_offset = segment[1] - source_index;
            let original_line_offset = segment[2] - original_line;
            let original_column_offset = segment[3] - original_column;

            generated_column = segment[0];
            source_index = segment[1];
            original_line = segment[2];
            original_column = segment[3];

            vec![
              generated_column_offset,
              source_index_offset,
              original_line_offset,
              original_column_offset,
            ]
          })
          .collect::<Line>()
      })
      .collect::<Mappings>();

    decoded_mappings
  }

  // generate encoded mappings, mappings are encoded relatively
  pub fn generate_encoded_mappings(decoded_mappings: &Mappings) -> Result<String> {
    let mut encoded_mappings: Vec<String> = vec![];

    for line in decoded_mappings.iter() {
      let mut line_str: Vec<String> = vec![];

      for segment in line.iter() {
        let mut segment_str: Vec<String> = vec![];

        for item in segment.iter() {
          let mut vlq_output: Vec<u8> = vec![];

          match vlq::encode(item.to_owned(), &mut vlq_output) {
            Err(e) => {
              return Err(Error::new_with_reason(
                MagicStringErrorType::VLQEncodingError,
                e.to_string().as_str(),
              ))
            }
            _ => (),
          };

          match String::from_utf8(vlq_output) {
            Err(_) => {
              return Err(Error::new(MagicStringErrorType::UTF8EncodingError));
            }
            Ok(str) => segment_str.push(str),
          }
        }

        line_str.push(segment_str.join(""));
      }

      encoded_mappings.push(line_str.join(","));
    }

    let encoded_mappings_str = encoded_mappings.join(";");

    Ok(encoded_mappings_str)
  }
}

#[cfg(test)]
mod tests {
  use super::Mapping;

  #[test]
  fn absolute_mapping_to_relative_mapping() {
    let mut mapping = Mapping::new();

    mapping.absolute_mappings.push(vec![vec![3, 0, 0, 0]]);
    mapping.absolute_mappings.push(vec![vec![0, 0, 1, 0]]);
    mapping.absolute_mappings.push(vec![vec![0, 0, 2, 0]]);

    let decoded_mappings = mapping.get_decoded_mappings();

    assert_eq!(
      &decoded_mappings,
      &vec![
        vec![vec![3, 0, 0, 0]],
        vec![vec![0, 0, 1, 0]],
        vec![vec![0, 0, 1, 0]],
      ]
    )
  }
}
