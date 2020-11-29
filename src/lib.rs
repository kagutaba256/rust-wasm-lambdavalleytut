mod utils;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod gl_setup;

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
pub struct Client {
  gl: GL,
}

#[wasm_bindgen]
impl Client {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();
    Self { gl: gl }
  }

  pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
    Ok(())
  }

  pub fn render(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
  }
}
