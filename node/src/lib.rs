#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

use magic_string::create as magic_string_create;

#[napi]
fn init(val: u32) -> u32 {
    magic_string_create();
    val
}
