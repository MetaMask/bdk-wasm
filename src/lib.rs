pub mod bitcoin;
pub mod types;
mod utils;

pub use utils::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    set_panic_hook();
}
