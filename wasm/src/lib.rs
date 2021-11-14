use wasm_bindgen::prelude::*;

use magic_string::create as magic_string_create;

#[wasm_bindgen]
pub fn create(val: u32) -> Result<u32, JsValue> {
    magic_string_create();
    Ok(val)
}
