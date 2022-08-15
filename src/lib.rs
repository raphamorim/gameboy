use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod gameboy;
pub mod screen;

mod cpu;
mod gpu;
mod input;
mod mmu;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    screen::web::render().await
}
