// tests are copied from `magic-string`: https://github.com/Rich-Harris/magic-string/blob/master/test/MagicString.js

#[cfg(test)]
mod pend {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_prepend_append() -> Result {
    let mut ms = MagicString::new("abc\ndef\ng");

    ms.append("123")?.prepend("456")?;

    println!("{:#?}", ms.generate_map().unwrap());

    assert_eq!(ms.to_string(), "456abc\ndef\ng123");
    assert_eq!(ms.generate_map().unwrap().mappings, "GAAA;AACA;AACA");

    Ok(())
  }

  #[test]
  fn should_append_left_right() -> Result {
    let mut ms = MagicString::new("abc\ndef\ng");

    ms.append_left(5, "123")?
      .prepend_left(6, "wu\nwu")?
      .append_right(8, "456")?
      .prepend_right(9, "hihi")?;

    println!("{:#?}", ms.generate_map().unwrap());
    println!("{:#?}", ms.generate_decoded_map());
    println!("{}", ms.to_string());
    // assert_eq!(ms.to_string(), "abc\nd123ef\n456g");
    // assert_eq!(ms.generate_map().unwrap().mappings, "AAAA;AACA,IAAC;GACD");

    Ok(())
  }
}
