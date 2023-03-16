#[cfg(test)]

mod slice {
  use magic_string::{MagicString, OverwriteOptions, Result};

  #[test]
  fn should_return_the_generated_content_between_the_specified_original_characters() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    assert_eq!(s.slice(3, 9)?, "defghi");
    s.overwrite(4, 8, "XX", OverwriteOptions::default())?;
    assert_eq!(s.slice(3, 9)?, "dXXi");
    s.overwrite(2, 10, "ZZ", OverwriteOptions::default())?;
    assert_eq!(s.slice(1, 11)?, "bZZk");
    assert_eq!(s.slice(2, 10)?, "ZZ");

    Ok(())
  }

  //   #[test]
  //   fn defaults_end_to_the_original_string_length() -> Result {
  //     let mut s = MagicString::new("abcdefghijkl");
  //     assert_eq!(s.slice(3)?, "defghijkl");
  //   }

  #[test]
  fn allow_negative_numbers_as_params() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    assert_eq!(s.slice(0, -3)?, "abcdefghi");
    // assert_eq!(s.slice(-3)?, "jkl");
    Ok(())
  }
  #[test]
  fn includes_inserted_characters_respecting_insertion_direction() -> Result {
    let mut s = MagicString::new("abefij");

    s.prepend_right(2, "cd")?;
    s.append_left(4, "gh")?;

    // assert_eq!(s.slice(), "abcdefghij");
    assert_eq!(s.slice(1, 5)?, "bcdefghi");
    assert_eq!(s.slice(2, 4)?, "cdefgh");
    assert_eq!(s.slice(3, 4)?, "fgh");
    assert_eq!(s.slice(0, 2)?, "ab");
    assert_eq!(s.slice(0, 3)?, "abcde");
    assert_eq!(s.slice(4, 6)?, "ij");
    assert_eq!(s.slice(3, 6)?, "fghij");
    Ok(())
  }

  // wating for move to be implemented
  //   #[test]
  //   fn supports_characters_moved_outward() -> Result {
  //     let mut s = MagicString::new("abcdEFghIJklmn");

  //     s._move(4, 6, 2)?;
  //     s._move(8, 10, 12)?;
  //     assert_eq!(s.to_string(), "abEFcdghklIJmn");

  //     assert_eq!(s.slice(1, -1)?, "bEFcdghklIJm");
  //     assert_eq!(s.slice(2, -2)?, "cdghkl");
  //     assert_eq!(s.slice(3, -3)?, "dghk");
  //     assert_eq!(s.slice(4, -4)?, "EFcdghklIJ");
  //     assert_eq!(s.slice(5, -5)?, "FcdghklI");
  //     assert_eq!(s.slice(6, -6)?, "gh");
  //     Ok(())
  //   }

  //   #[test]
  //   fn supports_characters_moved_inward() -> Result {
  //     let mut s = MagicString::new("abCDefghijKLmn");
  //     s._move(2, 4, 6)?;
  //     s._move(10, 12, 8)?;
  //     assert_eq!(s.to_string(), "abefCDghKLijmn");

  //     assert_eq!(s.slice(1, -1)?, "befCDghKLijm");
  //     assert_eq!(s.slice(2, -2)?, "CDghKL");
  //     assert_eq!(s.slice(3, -3)?, "DghK");
  //     assert_eq!(s.slice(4, -4)?, "efCDghKLij");
  //     assert_eq!(s.slice(5, -5)?, "fCDghKLi");
  //     assert_eq!(s.slice(6, -6)?, "gh");
  //     Ok(())
  //   }

  //   #[test]
  //   fn supports_characters_moved_inward() -> Result {
  //     let mut s = MagicString::new("abCDefghIJkl");
  //     // s._move(2, 4, 8)?;
  //     // s._move(8, 10, 4)?;
  //     assert_eq!(s.to_string(), "abIJefghCDkl");

  //     assert_eq!(s.slice(1, -1)?, "bIJefghCDk");
  //     assert_eq!(s.slice(2, -2)?, "");
  //     assert_eq!(s.slice(3, -3)?, "");
  //     assert_eq!(s.slice(-3, 3)?, "JefghC");
  //     assert_eq!(s.slice(4, -4)?, "efgh");
  //     assert_eq!(s.slice(0, 3)?, "abIJefghC");
  //     // assert_eq!(s.slice(3)?, "Dkl");
  //     assert_eq!(s.slice(0, -3)?, "abI");
  //     // assert_eq!(s.slice(-3)?, "JefghCDkl");
  //     Ok(())
  //   }
}
