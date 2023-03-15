#[cfg(test)]
mod move_to {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_move_from_start() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(0, 3, 6)?;
    assert_eq!(s.to_string(), "defabcghijkl");
    Ok(())
  }

  #[test]
  fn should_move_to_start() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(3, 6, 0)?;

    assert_eq!(s.to_string(), "defabcghijkl");
    Ok(())
  }

  #[test]
  fn should_move_from_end() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(9, 12, 3)?;

    assert_eq!(s.to_string(), "abcjkldefghi");
    Ok(())
  }
  #[test]
  fn should_move_to_end() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(3, 6, 12)?;

    assert_eq!(s.to_string(), "abcghijkldef");
    Ok(())
  }

  #[test]
  fn should_move_and_remove() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(3, 6, 12)?;
    s.move_to(3, 5, 0)?;

    assert_eq!(s.to_string(), "deabcghijklf");

    Ok(())
  }

  #[test]
  fn should_move_after_insert() -> Result {
    let mut s = MagicString::new("abcdefghijk");

    s.prepend("xyz")?;
    s.append("mn")?;
    s.prepend_left(4, "A")?;
    s.append_left(4, "B")?;
    s.prepend_right(4, "C")?;
    s.append_right(4, "D")?;
    s.move_to(0, 3, 6)?;
    assert_eq!(s.to_string(), "xyzdABCDefabcghijkmn");
    Ok(())
  }

  #[test]
  fn should_ignores_redundant_move() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.prepend_right(9, "X")?;
    s.move_to(9, 12, 6)?;
    s.append_left(12, "Y")?;
    s.move_to(6, 9, 12)?; // this is redundant – [6,9] is already after [9,12]

    assert_eq!(s.to_string(), "abcdefXjklYghi");

    Ok(())
  }

  #[test]
  fn should_move_content_to_middle() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.move_to(3, 6, 9)?;

    assert_eq!(s.to_string(), "abcghidefjkl");
    Ok(())
  }

  #[test]
  fn should_handles_multiple_moves_of_same_snippet() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.move_to(0, 3, 6)?;
    assert_eq!(s.to_string(), "defabcghijkl");

    s.move_to(0, 3, 9)?;
    assert_eq!(s.to_string(), "defghiabcjkl");

    Ok(())
  }
  #[test]
  fn should_handles_moves_of_adjacent_snippets() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.move_to(0, 2, 6)?;
    assert_eq!(s.to_string(), "cdefabghijkl");

    s.move_to(2, 4, 6)?;
    assert_eq!(s.to_string(), "efabcdghijkl");

    Ok(())
  }
  #[test]
  fn should_handles_moves_to_same_index() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.move_to(0, 2, 6)?.move_to(3, 5, 6)?;
    assert_eq!(s.to_string(), "cfabdeghijkl");

    Ok(())
  }
  #[test]
  fn should_allows_edits_of_moved_content() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.move_to(3, 6, 9)?;
    s.overwrite(3, 6, "DEF", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcghiDEFjkl");

    let mut s = MagicString::new("abcdefghijkl");

    s.move_to(3, 6, 9)?;
    s.overwrite(4, 5, "E", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "abcghidEfjkl");
    Ok(())
  }
  //   #[test]
  //   fn should_move_follows_inserts() -> Result {
  //     let mut s = MagicString::new("abcdefghijkl");
  //     s.move_to(3, 6, 9)?;

  //     assert_eq!(s.to_string(), "abcghidefjkl");
  //     Ok(())
  //   }
  #[test]
  fn should_moves_content_inserted_at_end_of_range() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.append_left(6, "X")?.move_to(3, 6, 9)?;

    assert_eq!(s.to_string(), "abcghidefXjkl");

    Ok(())
  }
  #[test]
  fn should_returns_this() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    let result = s.move_to(3, 6, 9)?;
    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);
    Ok(())
  }
}
