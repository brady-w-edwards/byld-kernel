use wasm_bindgen::prelude::*;

// Import the console.log function from the `console` API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

pub mod geometry;
pub mod topology;

// Re-export the geometry types and functions for WASM
pub use geometry::{Point3D, create_random_point, create_point_grid};

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Geometry Kernel loaded!");
}
