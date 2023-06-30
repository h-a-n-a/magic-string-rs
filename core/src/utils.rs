pub mod locator {
  #[allow(dead_code)]
  #[derive(Debug, Clone)]
  pub struct Locator {
    original_lines: Vec<String>,
    line_offsets: Vec<u32>,
  }

  type Location = (u32, u32);

  impl Locator {
    pub fn new(original: &str) -> Self {
      let original_lines = original
        .split('\n')
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

      let mut line_offsets: Vec<u32> = vec![];

      let mut pos_in_original = 0;
      for line in original_lines.iter() {
        line_offsets.push(pos_in_original);
        pos_in_original += line.len() as u32 + 1;
      }

      Locator {
        original_lines,
        line_offsets,
      }
    }

    pub fn locate(&self, index: u32) -> Location {
      let mut i = 0;
      let mut j = self.line_offsets.len();

      while i < j {
        let m = (i + j) >> 1;
        if index < self.line_offsets[m] {
          j = m;
        } else {
          i = m + 1;
        }
      }
      let line = (i - 1) as u32;
      let column = index - self.line_offsets[line as usize];

      (line, column)
    }
  }

  #[cfg(test)]
  mod tests {
    use super::Locator;

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
}

pub mod trim {
  use regex::Regex;

  use crate::Result;

  pub fn trim_start_regexp<'a>(s: &'a str, reg_pat: &'a str) -> Result<&'a str> {
    if s.is_empty() {
      return Ok(s);
    }

    let matcher = Regex::new(reg_pat)?;
    let chars = s.chars().collect::<Vec<_>>();

    let mut pos = 0;

    while pos < s.len() {
      let c = chars.get(pos).unwrap();
      if !matcher.is_match(c.to_string().as_str()) {
        break;
      }
      pos += 1;
    }

    Ok(&s[pos..])
  }

  pub fn trim_end_regexp<'a>(s: &'a str, reg_pat: &'a str) -> Result<&'a str> {
    if s.is_empty() {
      return Ok(s);
    }

    let matcher = Regex::new(reg_pat)?;
    let chars = s.chars().collect::<Vec<_>>();

    let mut pos = (s.len() - 1) as i32;

    while pos >= 0 {
      let c = chars.get(pos as usize).unwrap();
      if !matcher.is_match(c.to_string().as_str()) {
        break;
      }
      pos -= 1;
    }

    Ok(&s[..(pos + 1) as usize])
  }

  #[test]
  fn should_trim_start() -> Result {
    assert_eq!(trim_start_regexp("  abc  ", "\\s")?, "abc  ");
    assert_eq!(trim_start_regexp("\t\t\tabc\t\t", "\\t")?, "abc\t\t");
    assert_eq!(trim_start_regexp("\n\nabc\t\t", "\n")?, "abc\t\t");
    assert_eq!(trim_start_regexp("\n\n\n", "\n")?, "");

    Ok(())
  }

  #[test]
  fn should_trim_end() -> Result {
    assert_eq!(trim_end_regexp("  abc  ", "\\s")?, "  abc");
    assert_eq!(trim_end_regexp("\t\t\tabc\t\t", "\\t")?, "\t\t\tabc");
    assert_eq!(trim_end_regexp("\t\tabc\n\n", "\n")?, "\t\tabc");
    assert_eq!(trim_end_regexp("\n\n\n", "\n")?, "");

    Ok(())
  }

  #[test]
  fn should_not_trim_unrelated_contents() -> Result {
    assert_eq!(trim_start_regexp("\\s\\sabc", "\\s")?, "\\s\\sabc");
    assert_eq!(trim_end_regexp("abc\\t\\t", "\\t")?, "abc\\t\\t");

    Ok(())
  }
}

use crate::{Error, MagicStringErrorType, Result};

use regex::Regex;

pub fn normalize_index(s: &str, index: i64) -> Result<usize> {
  let len = s.len() as i64;

  let index = if index < 0 { index + len } else { index };

  if index < 0 || index > len {
    return Err(Error::new_with_reason(
      MagicStringErrorType::MagicStringOutOfRangeError,
      "index out of range",
    ));
  }

  Ok(index as usize)
}

pub fn guess_indent(str: &str) -> Result<String> {
  let lines: Vec<&str> = str.split('\n').collect();

  let tab_pattern = Regex::new(r"^\t+")?;
  let space_pattern = Regex::new(r"^ {2,}")?;

  let spaced = lines
    .clone()
    .into_iter()
    .filter(|line| space_pattern.is_match(line))
    .collect::<Vec<&str>>();
  let tabbed = lines
    .clone()
    .into_iter()
    .filter(|line| tab_pattern.is_match(line))
    .collect::<Vec<&str>>();

  if tabbed.len() == 0 && spaced.len() == 0 || tabbed.len() > spaced.len() {
    return Ok("\t".to_string());
  }

  let mut min: usize = 2 ^ 32;
  for space_line in spaced {
    let mut space_count = 0;
    for c in space_line.chars() {
      if c == ' ' {
        space_count += 1;
      } else {
        break;
      }
    }

    if space_count < min {
      min = space_count
    }
  }
  Ok(" ".repeat(min).to_string())
}
