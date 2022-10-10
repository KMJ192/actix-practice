use wasm_bindgen::{JsCast};
use crate::window;

use web_sys::{Window, Document, HtmlElement, HtmlCollection};

#[derive(Debug)]
pub struct Doc {
  window: Option<Window>,
  document: Option<Document>,
  main: Option<HtmlElement>
}

impl Doc {
  pub fn new() -> Self {
    Doc {
      window: None,
      document: None,
      main: None,
    }
  }

  pub fn set_window(&mut self) {
    self.window = Some(window().expect("window does not exist"));
  }

  pub fn set_document(&mut self) {
    if let Some(window) = &self.window {
      let doc = window.document().expect("expecting a document on window");
      self.document = Some(doc);
    }
  }

  pub fn get_main(&mut self) {
    if let Some(document) = &self.document {
      let main = 
        document
          .get_element_by_id("App")
          .unwrap()
          .dyn_into::<HtmlElement>()
          .unwrap();
      self.main = Some(main);
    }
  }

  pub fn get_element_by_id(&self, id: String) -> Option<HtmlElement> {
    if let Some(document) = &self.document {
      let node = 
        document
          .get_element_by_id(&id)
          .unwrap()
          .dyn_into::<HtmlElement>()
          .unwrap();
      return Some(node);
    }

    None
  }

  pub fn get_elements_by_class_names(&self, class_name: String) -> Option<HtmlCollection> {
    if let Some(document) = &self.main {
      let node_list = document.get_elements_by_class_name(&class_name);
      return Some(node_list);
    }

    None
  }

  pub fn create_element(&self, ele_type: String) -> Option<HtmlElement> {
    if let Some(document) = &self.document {
      let element =
        document
          .create_element(&ele_type)
          .unwrap()
          .dyn_into::<HtmlElement>()
          .unwrap();
      return Some(element);
    }

    None
  }

  pub fn test(&self) {
    if let Some(node) = &self.main {
      if let Some(new) = &self.create_element(String::from("div")) {
        node.append_child(new);
      };
    };
  }
}