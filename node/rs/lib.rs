extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{
  CallContext, Env, JsBoolean, JsNumber, JsObject, JsString, JsUndefined, Property, Result,
};

use magic_string::{GenerateDecodedMapOptions, MagicString};

// Even the fact that `core` library returned `self` for chaining,
// It's not performative for us to support this in the binding itself.

fn get_instance<'a>(ctx: &'a CallContext) -> Result<&'a mut MagicString> {
  let this: JsObject = ctx.this_unchecked();
  let instance: &mut MagicString = ctx.env.unwrap(&this)?;
  Ok(instance)
}

fn wrap_string_to_option(s: String) -> Option<String> {
  if s.is_empty() {
    None
  } else {
    Some(s)
  }
}

#[js_function(1)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
  let original_str = ctx.get::<JsString>(0)?.into_utf8()?;

  let mut this: JsObject = ctx.this_unchecked();
  ctx
    .env
    .wrap(&mut this, MagicString::new(original_str.as_str()?))?;

  ctx.env.get_undefined()
}

#[js_function(1)]
fn append(ctx: CallContext) -> Result<JsUndefined> {
  let str = ctx.get::<JsString>(0)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.append(str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(1)]
fn prepend(ctx: CallContext) -> Result<JsUndefined> {
  let str = ctx.get::<JsString>(0)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.prepend(str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn append_left(ctx: CallContext) -> Result<JsUndefined> {
  let index = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.append_left(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn prepend_left(ctx: CallContext) -> Result<JsUndefined> {
  let index = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.prepend_left(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn append_right(ctx: CallContext) -> Result<JsUndefined> {
  let index = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.append_right(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn prepend_right(ctx: CallContext) -> Result<JsUndefined> {
  let index = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.prepend_right(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(4)]
fn to_json_sourcemap(ctx: CallContext) -> Result<JsString> {
  let file = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source_root = ctx.get::<JsString>(1)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsString>(2)?.into_utf8()?.into_owned()?;
  let include_content = ctx.get::<JsBoolean>(3)?.get_value()?;

  let instance = get_instance(&ctx)?;
  let generated_map = instance.generate_map(GenerateDecodedMapOptions {
    file: wrap_string_to_option(file),
    source_root: wrap_string_to_option(source_root),
    source: wrap_string_to_option(source),
    include_content,
  })?;

  ctx.env.create_string(generated_map.to_json()?.as_str())
}

#[js_function(4)]
fn to_url_sourcemap(ctx: CallContext) -> Result<JsString> {
  let file = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source_root = ctx.get::<JsString>(1)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsString>(2)?.into_utf8()?.into_owned()?;
  let include_content = ctx.get::<JsBoolean>(3)?.get_value()?;

  let instance = get_instance(&ctx)?;
  let generated_map = instance.generate_map(GenerateDecodedMapOptions {
    file: wrap_string_to_option(file),
    source_root: wrap_string_to_option(source_root),
    source: wrap_string_to_option(source),
    include_content,
  })?;

  ctx.env.create_string(generated_map.to_url()?.as_str())
}

#[js_function(4)]
fn generate_decoded_map(ctx: CallContext) -> Result<JsObject> {
  let file = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source_root = ctx.get::<JsString>(1)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsString>(2)?.into_utf8()?.into_owned()?;
  let include_content = ctx.get::<JsBoolean>(3)?.get_value()?;

  let instance = get_instance(&ctx)?;
  let decoded_map = instance.generate_decoded_map(GenerateDecodedMapOptions {
    file: wrap_string_to_option(file),
    source_root: wrap_string_to_option(source_root),
    source: wrap_string_to_option(source),
    include_content,
  })?;

  let mut decoded_output = ctx.env.create_object()?;
  decoded_output.set_named_property("file", {
    if let Some(s) = decoded_map.file.to_owned() {
      ctx.env.create_string(s.as_str())?
    } else {
      ctx.env.create_string("")?
    }
  })?;
  decoded_output.set_named_property("sourceRoot", {
    if let Some(s) = decoded_map.source_root {
      ctx.env.create_string(s.as_str())?
    } else {
      ctx.env.create_string("")?
    }
  })?;
  decoded_output.set_named_property("names", {
    let mut names = ctx.env.create_array_with_length(decoded_map.names.len())?;
    for (i, e) in decoded_map.names.iter().enumerate() {
      names.set_element(i as u32, ctx.env.create_string(e.as_str())?)?;
    }
    names
  })?;
  decoded_output.set_named_property("sources", {
    let mut sources = ctx
      .env
      .create_array_with_length(decoded_map.sources.len())?;
    for (i, s) in decoded_map.sources.iter().enumerate() {
      if let Some(s) = s {
        sources.set_element(i as u32, ctx.env.create_string(s.as_str())?)?;
      } else {
        sources.set_element(i as u32, ctx.env.create_string("")?)?;
      }
    }
    sources
  })?;
  decoded_output.set_named_property("sourcesContent", {
    let mut sources_content = ctx
      .env
      .create_array_with_length(decoded_map.sources_content.len())?;
    for (i, s) in decoded_map.sources_content.iter().enumerate() {
      if let Some(s) = s {
        sources_content.set_element(i as u32, ctx.env.create_string(s.as_str())?)?;
      } else {
        sources_content.set_element(i as u32, ctx.env.create_string("")?)?;
      }
    }
    sources_content
  })?;
  decoded_output.set_named_property("mappings", {
    let mut mappings = ctx
      .env
      .create_array_with_length(decoded_map.mappings.len())?;
    for (i, l) in decoded_map.mappings.iter().enumerate() {
      let mut line = ctx.env.create_array_with_length(l.len())?;

      for (i, s) in l.iter().enumerate() {
        let mut segment = ctx.env.create_array_with_length(s.len())?;

        for (i, item) in s.iter().enumerate() {
          segment.set_element(i as u32, ctx.env.create_int64(item.to_owned())?)?;
        }

        line.set_element(i as u32, segment)?
      }

      mappings.set_element(i as u32, line)?;
    }

    mappings
  })?;

  Ok(decoded_output)
}

#[js_function]
fn to_string(ctx: CallContext) -> Result<JsString> {
  let instance = get_instance(&ctx)?;

  ctx.env.create_string(instance.to_string().as_str())
}

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let magic_string_class = env.define_class(
    "MagicString",
    constructor,
    &[
      Property::new(&env, "append")?.with_method(append),
      Property::new(&env, "prepend")?.with_method(prepend),
      Property::new(&env, "appendLeft")?.with_method(append_left),
      Property::new(&env, "appendRight")?.with_method(append_right),
      Property::new(&env, "prependLeft")?.with_method(prepend_left),
      Property::new(&env, "prependRight")?.with_method(prepend_right),
      Property::new(&env, "toJson")?.with_method(to_json_sourcemap),
      Property::new(&env, "toUrl")?.with_method(to_url_sourcemap),
      Property::new(&env, "toString")?.with_method(to_string),
      Property::new(&env, "generate_decoded_map")?.with_method(generate_decoded_map),
    ],
  )?;
  exports.set_named_property("MagicString", magic_string_class)?;

  Ok(())
}
