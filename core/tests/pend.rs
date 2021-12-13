#[cfg(test)]
mod pend {
  use magic_string::{MagicString, Result};

  #[test]
  fn preserves_intended_order() -> Result {
    let mut s = MagicString::new("0123456789");

    s.append_left(5, "A")?;
    s.prepend_right(5, "a")?;
    s.prepend_right(5, "b")?;
    s.append_left(5, "B")?;
    s.append_left(5, "C")?;
    s.prepend_right(5, "c")?;
    assert_eq!(s.to_string(), "01234ABCcba56789");

    s.append_right(7, "}")?;
    s.append_right(7, "{")?;
    assert_eq!(s.to_string(), "01234ABCcba56}{789");

    s.prepend_left(7, "]")?;
    s.prepend_left(7, "[")?;
    assert_eq!(s.to_string(), "01234ABCcba56[]}{789");

    Ok(())
  }

  #[test]
  fn preserves_intended_order_at_beginning_of_string() -> Result {
    let mut s = MagicString::new("x");

    s.append_left(0, "1")?;
    s.prepend_left(0, "2")?;
    s.append_left(0, "3")?;
    s.prepend_left(0, "4")?;

    assert_eq!(s.to_string(), "4213x");

    Ok(())
  }

  #[test]
  fn preserves_intended_order_at_end_of_string() -> Result {
    let mut s = MagicString::new("x");

    s.append_right(1, "1")?;
    s.prepend_right(1, "2")?;
    s.append_right(1, "3")?;
    s.prepend_right(1, "4")?;

    assert_eq!(s.to_string(), "x4213");

    Ok(())
  }
}

#[cfg(test)]
mod prepend {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_prepend_content() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.prepend("xyz")?;
    assert_eq!(s.to_string(), "xyzabcdefghijkl");

    s.prepend("123")?;
    assert_eq!(s.to_string(), "123xyzabcdefghijkl");

    Ok(())
  }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.prepend("s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}

#[cfg(test)]
mod prepend_left {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.prepend_left(0, "s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}

#[cfg(test)]
mod prepend_right {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.prepend_right(0, "s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}

#[cfg(test)]
mod append {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_prepend_content() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.append("xyz")?;
    assert_eq!(s.to_string(), "abcdefghijklxyz");

    s.append("123")?;
    assert_eq!(s.to_string(), "abcdefghijklxyz123");

    Ok(())
  }

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.prepend("s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}

#[cfg(test)]
mod append_left {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.append_left(0, "s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}

#[cfg(test)]
mod append_right {
  use magic_string::{MagicString, Result};

  #[test]
  fn should_return_self() -> Result {
    let mut s = MagicString::new("");
    let result = s.append_right(0, "s")?;

    let result_ptr = result as *mut _;
    let s_ptr = &s as *const _;

    assert_eq!(s_ptr, result_ptr);

    Ok(())
  }
}
