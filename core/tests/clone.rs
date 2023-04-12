#[cfg(test)]
mod remove {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_clone_a_magic_string() -> Result {

    let mut s = MagicString::new("abcdefghijkl");
    s.overwrite(3, 9, "XYZ", OverwriteOptions::default())?;
    let mut c = s.clone().unwrap();
    c.overwrite(3, 9, "XYZB", OverwriteOptions::default())?;

    assert_eq!(s.to_string(), "abcXYZjkl");
    assert_eq!(c.to_string(), "abcXYZBjkl");

    Ok(())
  }

  #[test]
  fn should_clone_intro_and_outro() -> Result {

    let mut s = MagicString::new("defghi");
    s.prepend("abc")?;
    s.append("jkl")?;
    let c = s.clone().unwrap();

    assert_eq!(s.to_string(), c.to_string());

    Ok(())
  }
}
