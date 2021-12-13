#[cfg(test)]
mod remove {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_remove_characters_from_the_original_string() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(1, 5)?;
    assert_eq!(s.to_string(), "afghijkl");

    s.remove(9, 12)?;
    assert_eq!(s.to_string(), "afghi");

    Ok(())
  }

  #[test]
  fn should_remove_from_the_start() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(0, 6)?;
    assert_eq!(s.to_string(), "ghijkl");

    Ok(())
  }

  #[test]
  fn should_remove_from_the_end() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(6, 12)?;
    assert_eq!(s.to_string(), "abcdef");

    Ok(())
  }

  #[test]
  fn should_treat_zero_length_removals_as_a_no_op() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(0, 0)?;
    s.remove(6, 6)?;
    s.remove(9, -3)?;
    assert_eq!(s.to_string(), "abcdefghijkl");

    Ok(())
  }

  #[test]
  fn should_remove_overlapping_ranges() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(3, 7)?;
    s.remove(5, 9)?;
    assert_eq!(s.to_string(), "abcjkl");

    let mut s = MagicString::new("abcdefghijkl");

    s.remove(3, 7)?;
    s.remove(4, 6)?;
    assert_eq!(s.to_string(), "abchijkl");

    Ok(())
  }

  #[test]
  fn should_remove_overlapping_ranges_redux() -> Result {
    let mut s = MagicString::new("abccde");

    s.remove(2, 3)?; // c
    s.remove(1, 3)?; // bc
    assert_eq!(s.to_string(), "acde");

    Ok(())
  }

  #[test]
  fn should_remove_modified_ranges() -> Result {
    let mut s = MagicString::new("abcdefghi");

    s.overwrite(3, 6, "DEF", OverwriteOptions::default())?;
    s.remove(2, 7)?; // cDEFg

    // assert_eq!(s.slice(1, 8), "bh"); // To be implemented
    assert_eq!(s.to_string(), "abhi");

    Ok(())
  }

  #[test]
  fn should_not_remove_content_inserted_after_the_end_of_removed_range() -> Result {
    let mut s = MagicString::new("ab.c;");

    s.prepend_right(0, "(")?;
    s.prepend_right(4, ")")?;
    s.remove(2, 4)?;

    assert_eq!(s.to_string(), "(ab);");

    Ok(())
  }

  #[test]
  fn should_remove_interior_inserts() -> Result {
    let mut s = MagicString::new("abc;");

    s.append_left(1, "[")?;
    s.prepend_right(1, "(")?;
    s.append_left(2, ")")?;
    s.prepend_right(2, "]")?;

    s.remove(1, 2)?;

    assert_eq!(s.to_string(), "a[]c;");

    Ok(())
  }

  // #[test]
  // fn should_remove_across_moved_content() -> Result {
  //   let mut s = MagicString::new("abcdefghijkl");
  //
  //   // to be implemented
  //   // s.move(6, 9, 3);
  //   s.remove(5, 7)?;
  //
  //   assert.equal(s.toString(), "abchidejkl");
  //
  //   Ok(())
  // }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.remove(0, 0)?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    let mut s = MagicString::new("abcdefg");
    let result = s.remove(1, 2)?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}
