use std::convert::TryInto;
extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Either, Env, JsNumber, JsObject, JsString, JsUndefined, Property, Result};

use magic_string::{DecodedMap, GenerateDecodedMapOptions, MagicString, MagicStringErrorType};

// Even the fact that `core` library returned `self` for chaining,
// It's not performative for us to support this in the binding itself.

fn get_instance<'a>(ctx: &'a CallContext) -> Result<&'a mut MagicString> {
  let this: JsObject = ctx.this_unchecked();
  let instance: &mut MagicString = ctx.env.unwrap(&this)?;
  Ok(instance)
}

#[js_function(1)]
fn magic_string_ctor(ctx: CallContext) -> Result<JsUndefined> {
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
  let index: usize = ctx.get::<JsNumber>(0)?.try_into()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.append_left(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(1)]
fn prepend_left(ctx: CallContext) -> Result<JsUndefined> {
  let index: usize = ctx.get::<JsNumber>(0)?.try_into()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.prepend_left(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn append_right(ctx: CallContext) -> Result<JsUndefined> {
  let index: usize = ctx.get::<JsNumber>(0)?.try_into()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.append_right(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn prepend_right(ctx: CallContext) -> Result<JsUndefined> {
  let index: usize = ctx.get::<JsNumber>(0)?.try_into()?;
  let str = ctx.get::<JsString>(1)?.into_utf8()?;

  let instance = get_instance(&ctx)?;
  instance.prepend_right(index, str.as_str()?)?;

  ctx.env.get_undefined()
}

#[js_function(1)]
fn to_json_sourcemap(ctx: CallContext) -> Result<JsString> {
  let options: GenerateDecodedMapOptions = ctx.get::<JsObject>(0)?.try_into()?;

  let instance = get_instance(&ctx)?;
  let generated_map = instance.generate_map(options)?;

  let json = generated_map.to_json()?;

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

  ctx.env.create_string(json.as_str())
}

#[js_function(1)]
fn to_url_sourcemap(ctx: CallContext) -> Result<JsString> {
  let options: GenerateDecodedMapOptions = ctx.get::<JsObject>(0)?.try_into()?;

  let instance = get_instance(&ctx)?;
  let generated_map = instance.generate_map(options)?;

  let url = generated_map.to_url()?;

  ctx.env.create_string(url.as_str())
}

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let magic_string_class = env.define_class(
    "MagicString",
    magic_string_ctor,
    &[
      Property::new(&env, "append")?.with_method(append),
      Property::new(&env, "prepend")?.with_method(prepend),
      Property::new(&env, "append_left")?.with_method(append_left),
      Property::new(&env, "append_right")?.with_method(append_right),
      Property::new(&env, "prepend_left")?.with_method(prepend_left),
      Property::new(&env, "prepend_right")?.with_method(prepend_right),
      Property::new(&env, "to_json")?.with_method(to_json_sourcemap),
      Property::new(&env, "to_url_sourcemap")?.with_method(to_url_sourcemap),
    ],
  )?;
  exports.set_named_property("MagicString", magic_string_class)?;

  Ok(())
}
