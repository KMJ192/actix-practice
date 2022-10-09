use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::*;

#[wasm_bindgen]
pub fn app() {
  console::log_1(&"test123".into());
}
