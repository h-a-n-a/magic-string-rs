#[cfg(test)]
mod trim {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_trim_original_content() -> Result {
    assert_eq!(
      MagicString::new("   abcdefghijkl   ")
        .trim(None)?
        .to_string(),
      "abcdefghijkl"
    );

    assert_eq!(
      MagicString::new("   abcdefghijkl").trim(None)?.to_string(),
      "abcdefghijkl"
    );

    assert_eq!(
      MagicString::new("abcdefghijkl   ").trim(None)?.to_string(),
      "abcdefghijkl"
    );

    Ok(())
  }

  #[test]
  fn should_trim_replaced_content() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.overwrite(0, 3, "     ", OverwriteOptions::default())?;
    s.overwrite(9, 12, "     ", OverwriteOptions::default())?;

    s.trim(None)?;

    assert_eq!(s.to_string(), "defghi");

    Ok(())
  }

  #[test]
  fn should_trim_prepended_appended_content() -> Result {
    let mut s = MagicString::new(" abcdefghijkl ");

    s.prepend("     ")?;
    s.append("     ")?;
    s.trim(None)?;

    assert_eq!(s.to_string(), "abcdefghijkl");

    Ok(())
  }

  #[test]
  fn should_trim_empty_string() -> Result {
    let mut s = MagicString::new("     ");

    s.trim(None)?;

    assert_eq!(s.to_string(), "");

    Ok(())
  }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("   ");

    let result = s.trim(None)?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }

  #[test]
  fn should_support_trimming_chunks_with_intro_and_outro() -> Result {
    let mut s = MagicString::new("    \n");
    s.append_right(4, "test")?;
    s.trim(None)?;

    assert_eq!(s.to_string(), "test");

    Ok(())
  }

  #[test]
  fn should_support_trimming_with_given_pattern() -> Result {
    let mut s = MagicString::new("  \n\n\t abc \n\t ");
    s.trim(Some("\\s|\t"))?;

    assert_eq!(s.to_string(), "abc");

    Ok(())
  }
}

#[cfg(test)]
mod trim_lines {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_trim_original_content() -> Result {
    let mut s = MagicString::new("\n\n   abcdefghijkl   \n\n");
    s.trim_lines()?;

    assert_eq!(s.to_string(), "   abcdefghijkl   ");

    Ok(())
  }
}
