// tests are copied from `magic-string`: https://github.com/Rich-Harris/magic-string/blob/master/test/MagicString.js

#[cfg(test)]
mod append {
    use magic_string::MagicString;

    #[test]
    fn should_append() -> Result<(), ()> {
        let mut ms = MagicString::new("abcdefg");

        ms.append("123")?.prepend("456")?;

        assert_eq!(ms.to_string(), "456abcdefg123");

        Ok(())
    }
}