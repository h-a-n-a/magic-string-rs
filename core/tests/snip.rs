#[cfg(test)]
mod remove {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_return_a_clone_with_content_outside_start_and_end_removed() -> Result {

    let mut s = MagicString::new("abcdefghijkl");
    s.overwrite(6, 9, "GHI", OverwriteOptions::default())?;
    let snippet = s.snip(3, 9)?;

    assert_eq!(snippet.to_string(), "defGHI");

    Ok(())
  }

  #[test]
  fn should_snip_from_the_start() -> Result {

    let mut s = MagicString::new("abcdefghijkl");
    let snippet = s.snip(0, 6)?;

    assert_eq!(snippet.to_string(), "abcdef");

    Ok(())
  }

  #[test]
  fn should_snip_from_the_end() -> Result {

    let mut s = MagicString::new("abcdefghijkl");
    let snippet = s.snip(6, 12)?;

    assert_eq!(snippet.to_string(), "ghijkl");

    Ok(())
  }

  #[test]
  fn should_respect_original_indices() -> Result {

    let mut s = MagicString::new("abcdefghijkl");
    let mut snippet = s.snip(3, 9)?;

    snippet.overwrite(6, 9, "GHI", OverwriteOptions::default())?;
    assert_eq!(snippet.to_string(), "defGHI");

    Ok(())
  }
}
