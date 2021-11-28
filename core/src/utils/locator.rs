#[derive(Debug)]
pub struct Locator {
  original_lines: Vec<String>,
  line_offsets: Vec<usize>,
}

impl Locator {
  pub fn new(original: &str) -> Self {
    let original_lines = original
      .split("\n")
      .map(|line| line.to_owned())
      .collect::<Vec<String>>();

    let mut line_offsets: Vec<usize> = vec![];

    let mut pos_in_original = 0usize;
    for line in original_lines.iter() {
      line_offsets.push(pos_in_original);
      pos_in_original += line.len() + 1;
    }

    Locator {
      original_lines,
      line_offsets,
    }
  }

  pub fn locate(&self, index: usize) -> (usize, usize) {
    let mut i = 0usize;
    let mut j = self.line_offsets.len();

    while i < j {
      let m = (i + j) >> 1;
      if index < self.line_offsets[m] {
        j = m;
      } else {
        i = m + 1;
      }
    }
    let line = i - 1;
    let column = index - self.line_offsets[line];

    (line, column)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let locator = Locator::new("magic\nstring\nrs");

    assert_eq!(locator.original_lines[0], "magic");
    assert_eq!(locator.original_lines[1], "string");
    assert_eq!(locator.original_lines[2], "rs");

    assert_eq!(locator.line_offsets[0], 0);
    assert_eq!(locator.line_offsets[1], 6);
    assert_eq!(locator.line_offsets[2], 13);

    assert_eq!(locator.locate(2), (0, 2));
    assert_eq!(locator.locate(8), (1, 2));
    assert_eq!(locator.locate(14), (2, 1));
  }
}
