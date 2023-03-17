#[cfg(test)]

mod indent {
  use magic_string::{MagicString, Result};
  #[test]
  fn should_indent_content_with_a_single_tab_character_by_default() -> Result {
    let mut s = MagicString::new("abc\ndef\nghi\njkl");

    s.indent()?;
    assert_eq!(s.to_string(), "\tabc\n\tdef\n\tghi\n\tjkl");

    s.indent()?;
    assert_eq!(s.to_string(), "\t\tabc\n\t\tdef\n\t\tghi\n\t\tjkl");

    Ok(())
  }

  #[test]
  fn should_indent_content_using_existing_indentation_as_a_guide() -> Result {
    let mut s = MagicString::new("abc\n  def\n    ghi\n  jkl");

    s.indent()?;
    assert_eq!(s.to_string(), "  abc\n    def\n      ghi\n    jkl");

    s.indent()?;
    assert_eq!(s.to_string(), "    abc\n      def\n        ghi\n      jkl");

    Ok(())
  }

  #[test]
  fn should_disregard_single_space_indentation_when_auto_indenting() -> Result {
    let mut s = MagicString::new("abc\n/**\n *comment\n */");

    s.indent()?;

    assert_eq!(s.to_string(), "\tabc\n\t/**\n\t *comment\n\t */");
    Ok(())
  }
}
