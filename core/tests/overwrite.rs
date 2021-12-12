#[cfg(test)]
mod overwrite {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_replace_characters() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.overwrite(5, 8, "FGH", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcdeFGHijkl");

    Ok(())
  }

  #[test]
  #[should_panic]
  fn should_panic_if_overlapping_replacements_are_attempted() {
    let mut s = MagicString::new("abcdefghijkl");

    s.overwrite(7, 11, "xx", OverwriteOptions::default());
    s.overwrite(8, 12, "yy", OverwriteOptions::default())
      .expect_err("Error");
  }

  #[test]
  fn should_allow_contiguous_but_non_overlapping_replacements() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.overwrite(3, 6, "DEF", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcDEFghijkl");

    s.overwrite(6, 9, "GHI", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcDEFGHIjkl");

    s.overwrite(0, 3, "ABC", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "ABCDEFGHIjkl");

    s.overwrite(9, 12, "JKL", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "ABCDEFGHIJKL");

    Ok(())
  }

  #[test]
  fn replaces_zero_length_inserts_inside_overwrite() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.append_left(6, "XXX")?;
    s.overwrite(3, 9, "DEFGHI", OverwriteOptions::default())?;

    assert_eq!(s.to_string(), "abcDEFGHIjkl");

    Ok(())
  }

  #[test]
  fn replaces_non_zero_length_inserts_inside_overwrite() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.overwrite(3, 4, "XXX", OverwriteOptions::default())?;
    s.overwrite(3, 5, "DE", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcDEfghijkl");

    s.overwrite(7, 8, "YYY", OverwriteOptions::default())?;
    s.overwrite(6, 8, "GH", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcDEfGHijkl");

    Ok(())
  }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    let result = s.overwrite(3, 4, "D", OverwriteOptions::default())?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}
