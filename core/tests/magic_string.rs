// tests are copied from `magic-string`: https://github.com/Rich-Harris/magic-string/blob/master/test/MagicString.js

#[cfg(test)]
mod append_prepend {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_append_and_prepend_contents() -> Result {
    let mut ms = MagicString::new("AbcdefghijkL");

    ms.prepend("xyz")?;
    ms.append("xyz")?;
    ms.append("opq")?;
    ms.prepend("opq")?;

    assert_eq!(ms.to_string(), "xyzopqAbcdefghijkLxyzopq");
    assert_eq!(ms.generate_map().unwrap().mappings, "MAAA");

    Ok(())
  }

  #[test]
  fn should_do_chaining() -> Result {
    let mut ms = MagicString::new("123");

    ms.prepend("9")?.append("8")?.append("7")?.prepend("6")?;

    assert_eq!(ms.to_string(), "9612387");

    Ok(())
  }
}
