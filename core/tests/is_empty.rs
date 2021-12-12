#[cfg(test)]
mod empty {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_support_is_empty() -> Result {
    let str = " abcde   fghijkl ";
    let mut s = MagicString::new(str);

    assert_eq!(s.is_empty(), false);

    s.prepend("  ")?;

    s.overwrite(0, str.len() as i64, "", OverwriteOptions::default())?;

    assert_eq!(s.is_empty(), true);

    Ok(())
  }
}
