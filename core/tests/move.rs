#[cfg(test)]
mod move_to {
  use magic_string::MagicString;

  #[test]
  fn should_move_characters() {
    let mut s = MagicString::new("abcdefghijklmn");

    s.move_to(0, 3, 6);
    assert_eq!(s.to_string(), "defabcghijklmn")
  }
}
