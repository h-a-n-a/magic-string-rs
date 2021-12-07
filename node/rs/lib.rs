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

fn wrap_option_string(s: String) -> Option<String> {
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
    file: wrap_option_string(file),
    source_root: wrap_option_string(source_root),
    source: wrap_option_string(source),
    include_content,
  })?;

  ctx.env.create_string(generated_map.to_json()?.as_str())

  // let mut map_object = ctx.env.create_object()?;
  // map_object.set_named_property("version", ctx.env.create_uint32(3)?);
  //
  // let mut property_sources = ctx.env.create_array()?;
  // for (index, source) in generated_map.sources.iter().enumerate() {
  //   property_sources.set_element(index as u32, source);
  // }
  // map_object.set_named_property("sources", property_sources)?;
  //
  // let mut property_sources_content = ctx.env.create_array()?;
  // for (index, source_content) in generated_map.sources_content.iter().enumerate() {
  //   property_sources_content.set_element(index as u32, source_content);
  // }
  // map_object.set_named_property("sources_content", property_sources_content);
}

#[js_function(4)]
fn to_url_sourcemap(ctx: CallContext) -> Result<JsString> {
  let file = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source_root = ctx.get::<JsString>(1)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsString>(2)?.into_utf8()?.into_owned()?;
  let include_content = ctx.get::<JsBoolean>(3)?.get_value()?;

  let instance = get_instance(&ctx)?;
  let generated_map = instance.generate_map(GenerateDecodedMapOptions {
    file: wrap_option_string(file),
    source_root: wrap_option_string(source_root),
    source: wrap_option_string(source),
    include_content,
  })?;

  ctx.env.create_string(generated_map.to_url()?.as_str())
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
    ],
  )?;
  exports.set_named_property("MagicString", magic_string_class)?;

  Ok(())
}
