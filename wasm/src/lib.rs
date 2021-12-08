use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create(val: u32) -> Result<u32, JsValue> {
  Ok(val)
}
