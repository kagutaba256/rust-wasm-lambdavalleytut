mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  let cool = format!("when ur {}-ing really hard, so hard ur yolo'd", name);
  alert(&cool);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
  log("howdy... from rust");
}
