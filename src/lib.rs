mod utils;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

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
  program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl Client {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();
    Self {
      program_color_2d: programs::Color2D::new(&gl),
      gl: gl,
    }
  }

  pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, height, width);
    Ok(())
  }

  pub fn render(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    let curr_state = app_state::get_curr_state();
    self.program_color_2d.render(
      &self.gl, //gl
      0.,       //bottom
      100.,     //top
      0.,       //left
      100.,     //right
      curr_state.canvas_height,
      curr_state.canvas_width,
    );
  }
}
