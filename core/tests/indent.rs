#[cfg(test)]

mod indent {
  use magic_string::{IndentOptions, MagicString, OverwriteOptions, Result};
  #[test]
  fn should_indent_content_with_a_single_tab_character_by_default() -> Result {
    let mut s = MagicString::new("abc\ndef\nghi\njkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "\tabc\n\tdef\n\tghi\n\tjkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "\t\tabc\n\t\tdef\n\t\tghi\n\t\tjkl");

    Ok(())
  }

  #[test]
  fn should_indent_content_using_existing_indentation_as_a_guide() -> Result {
    let mut s = MagicString::new("abc\n  def\n    ghi\n  jkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "  abc\n    def\n      ghi\n    jkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "    abc\n      def\n        ghi\n      jkl");

    Ok(())
  }

  #[test]
  fn should_disregard_single_space_indentation_when_auto_indenting() -> Result {
    let mut s = MagicString::new("abc\n/**\n *comment\n */");

    s.indent(IndentOptions::default())?;

    assert_eq!(s.to_string(), "\tabc\n\t/**\n\t *comment\n\t */");
    Ok(())
  }

  #[test]
  fn should_indent_content_using_the_supplied_indent_string() -> Result {
    let mut s = MagicString::new("abc\ndef\nghi\njkl");
    s.indent(IndentOptions {
      indent_str: ">>".to_string(),
      ..IndentOptions::default()
    })?;
    assert_eq!(s.to_string(), ">>abc\n>>def\n>>ghi\n>>jkl");
    Ok(())
  }

  #[test]
  fn should_prevent_excluded_characters_from_being_indented() -> Result {
    let mut s = MagicString::new("abc\ndef\nghi\njkl");

    s.indent(IndentOptions {
      indent_str: String::from("  "),
      exclude: vec![7, 15],
    })?;
    assert_eq!(s.to_string(), "  abc\n  def\nghi\njkl");
    s.indent(IndentOptions {
      indent_str: String::from(">>"),
      exclude: vec![7, 15],
    })?;
    assert_eq!(s.to_string(), ">>  abc\n>>  def\nghi\njkl");
    Ok(())
  }

  #[test]
  fn should_not_add_characters_to_empty_line() -> Result {
    let mut s = MagicString::new("\n\nabc\ndef\n\nghi\njkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "\n\n\tabc\n\tdef\n\n\tghi\n\tjkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "\n\n\t\tabc\n\t\tdef\n\n\t\tghi\n\t\tjkl");
    Ok(())
  }

  #[test]
  fn should_not_add_characters_to_empty_lines_even_on_windows() -> Result {
    let mut s = MagicString::new("\r\n\r\nabc\r\ndef\r\n\r\nghi\r\njkl");

    s.indent(IndentOptions::default())?;
    assert_eq!(
      s.to_string(),
      "\r\n\r\n\tabc\r\n\tdef\r\n\r\n\tghi\r\n\tjkl"
    );

    s.indent(IndentOptions::default())?;
    assert_eq!(
      s.to_string(),
      "\r\n\r\n\t\tabc\r\n\t\tdef\r\n\r\n\t\tghi\r\n\t\tjkl"
    );
    Ok(())
  }

  #[test]
  fn should_indent_content_with_removals() -> Result {
    let mut s = MagicString::new("/* remove this line */\nvar foo = 1;");

    s.remove(0, 23)?;
    s.indent(IndentOptions::default())?;

    assert_eq!(s.to_string(), "\tvar foo = 1;");
    Ok(())
  }

  #[test]
  fn should_not_indent_patches_in_the_middle_of_a_line() -> Result {
    let mut s = MagicString::new("class Foo extends Bar {}");

    s.overwrite(18, 21, "Baz", OverwriteOptions::default())?;
    assert_eq!(s.to_string(), "class Foo extends Baz {}");

    s.indent(IndentOptions::default())?;
    assert_eq!(s.to_string(), "\tclass Foo extends Baz {}");
    Ok(())
  }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    let result = s.indent(IndentOptions::default())?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);
    Ok(())
  }
}
