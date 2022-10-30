use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod module;
use module::document::document::Documents;
use module::canvas::canvas::*;

#[wasm_bindgen]
pub fn app() {
  // let mut doc = Documents::new();
  // doc.set_window();
  // doc.set_document();
  // doc.get_main();
  // doc.test();
  // let test = format!("{:#?}", doc.get_elements_by_class_names(String::from("test")));
  // console::log_1(&test.into());
  canvas_test();
  console::log_1(&"test2".into());
}
