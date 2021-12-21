#[cfg(test)]
mod sourcemap {
  use magic_string::{GenerateDecodedMapOptions, MagicString, OverwriteOptions, Result};

  #[test]
  fn should_generate_a_sourcemap() -> Result {
    let mut s = MagicString::new("abcdefghijkl");
    s.remove(3, 9)?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      file: Some("output.md".to_owned()),
      source_root: Some("./".to_owned()),
      source: Some("input.md".to_owned()),
      include_content: true,
      hires: false,
    })?;

    assert_eq!(map.version, 3);
    assert_eq!(map.file, Some("output.md".to_owned()));
    assert_eq!(map.sources, vec![Some("input.md".to_owned())]);
    assert_eq!(map.source_root, Some("./".to_owned()));
    assert_eq!(map.sources_content, vec![Some("abcdefghijkl".to_owned())]);
    assert_eq!(map.mappings, "AAAA,GAAS".to_owned());

    assert_eq!(map.to_string().unwrap(), "{\"version\":3,\"mappings\":\"AAAA,GAAS\",\"names\":[],\"sources\":[\"input.md\"],\"sourcesContent\":[\"abcdefghijkl\"],\"file\":\"output.md\",\"sourceRoot\":\"./\"}".to_owned());
    assert_eq!(map.to_url().unwrap(), "data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJtYXBwaW5ncyI6IkFBQUEsR0FBUyIsIm5hbWVzIjpbXSwic291cmNlcyI6WyJpbnB1dC5tZCJdLCJzb3VyY2VzQ29udGVudCI6WyJhYmNkZWZnaGlqa2wiXSwiZmlsZSI6Im91dHB1dC5tZCIsInNvdXJjZVJvb3QiOiIuLyJ9".to_owned());

    let map = s.generate_map(GenerateDecodedMapOptions {
      file: Some("output.md".to_owned()),
      source_root: Some("./".to_owned()),
      source: Some("input.md".to_owned()),
      include_content: true,
      hires: true,
    })?;

    println!("{}{}", map.to_string()?, s.to_string());
    assert_eq!(map.mappings, "AAAA,CAAC,CAAC,CAAO,CAAC,CAAC".to_owned());

    Ok(())
  }

  #[test]
  fn should_generate_a_correct_sourcemap_for_prepend_content_when_hires_equals_to_false() -> Result
  {
    let mut s = MagicString::new("x\nq");

    s.prepend("y\n")?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    assert_eq!(map.mappings, ";AAAA;AACA");

    Ok(())
  }

  // TODO: support '\n' mappings
  // #[test]
  // fn should_generate_a_correct_sourcemap_for_prepend_content_when_hires_equals_to_true() -> Result {
  //   let mut s = MagicString::new("x\nq");
  //
  //   s.prepend("y\n")?;
  //
  //   let map = s.generate_map(GenerateDecodedMapOptions {
  //     include_content: true,
  //     hires: true,
  //     ..GenerateDecodedMapOptions::default()
  //   })?;
  //
  //   println!("{}", s.to_string());
  //   assert_eq!(map.mappings, ";AAAA,CAAC;AACD");
  //
  //   Ok(())
  // }

  #[test]
  fn should_correctly_map_inserted_content() -> Result {
    let mut s = MagicString::new("function Foo () {}");

    s.overwrite(9, 12, "Bar", OverwriteOptions::default())?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    assert_eq!(map.to_string().unwrap(), "{\"version\":3,\"mappings\":\"AAAA,SAAS,GAAG\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}".to_owned());

    Ok(())
  }

  #[test]
  fn should_correctly_map_inserted_multi_lines_content() -> Result {
    let mut s = MagicString::new("function Foo () {}");

    s.overwrite(15, 16, "\n", OverwriteOptions::default())?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    assert_eq!(map.to_string().unwrap(), "{\"version\":3,\"mappings\":\"AAAA;AAAgB\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}".to_owned());

    let mut s = MagicString::new("function Foo () {}");

    s.overwrite(
      15,
      17,
      " {\n  console.log(\"bar\")\n",
      OverwriteOptions::default(),
    )?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    assert_eq!(map.to_string().unwrap(), "{\"version\":3,\"mappings\":\"AAAA,eAAe;AAAA;AAAE\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}".to_owned());

    Ok(())
  }

  #[test]
  fn should_generate_one_segment_per_replacement() -> Result {
    let mut s = MagicString::new("var answer = 42");

    s.overwrite(4, 10, "number", OverwriteOptions::default())?;

    let options = GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    };

    let map = s.generate_decoded_map(options.to_owned())?;

    assert_eq!(map.mappings.len(), 1);
    assert_eq!(map.mappings.get(0).unwrap().len(), 3);

    let map = s.generate_map(options)?;

    assert_eq!(map.mappings.split(",").collect::<Vec<_>>().len(), 3);

    Ok(())
  }

  #[test]
  fn skips_empty_segments_at_the_start() -> Result {
    let mut s = MagicString::new("abcdefghijkl");

    s.remove(0, 3)?;
    s.remove(3, 6)?;

    let map = s.generate_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    let decoded_map = s.generate_decoded_map(GenerateDecodedMapOptions {
      include_content: true,
      ..GenerateDecodedMapOptions::default()
    })?;

    assert_eq!(map.mappings, "AAAM".to_owned());
    assert_eq!(decoded_map.mappings, vec![vec![vec![0, 0, 0, 6]]]);

    Ok(())
  }

  #[test]
  fn should_correctly_generate_a_map_with_trimmed_content() -> Result {
    let mut s = MagicString::new("abcdefghijkl ");
    s.trim(None)?;

    let map = s.generate_map(GenerateDecodedMapOptions::default())?;

    assert_eq!(map.mappings, "AAAA");

    let mut s = MagicString::new(" abcdefghijkl");
    s.trim(None)?;

    let map = s.generate_map(GenerateDecodedMapOptions::default())?;

    // This should be "AAAC" if we want to make `trim` more precise.
    assert_eq!(map.mappings, "AAAA");

    Ok(())
  }

  #[test]
  fn should_yield_consistent_result_between_append_left_and_prepend_right() -> Result {
    let mut s1 = MagicString::new("abcdefghijkl");
    s1.append_left(6, "X")?;
    let mut s2 = MagicString::new("abcdefghijkl");
    s2.prepend_right(6, "X")?;

    assert_eq!(
      s1.generate_map(GenerateDecodedMapOptions::default())?
        .to_string(),
      s2.generate_map(GenerateDecodedMapOptions::default())?
        .to_string()
    );

    Ok(())
  }
}
