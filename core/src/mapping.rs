pub struct Mapping {
  generated_code_line: usize,
  generated_code_column: usize,
}

impl Mapping {
  pub fn new() -> Self {
    Self {
      generated_code_line: 0,
      generated_code_column: 0,
    }
  }

  pub fn add_edited(&mut self) {
    todo!()
  }

  pub fn add_unedited_chunk(&mut self) {
    todo!()
  }

  pub fn advance(&mut self, str: &str) {
    let lines = str.split("\n");

    lines.for_each(|segment| {
      // save starting line for next op
      self.generated_code_line += 1;
    });

    // save starting column for next op
    self.generated_code_column = lens.len();
  }
}
