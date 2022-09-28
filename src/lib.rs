use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod gameboy;
mod gpu;
mod input;
mod mmu;
mod screen;

#[wasm_bindgen(start)]
pub async fn start() {
    // screen::web::render().await
    // TODO: Process GL
}

// #[cfg(test)]
// mod tests {
//     use crate::screen::web::render;
// use crate::gameboy::Gameboy;
//     use crate::screen;

//     #[test]
//     fn test_image_rendering() {
//         let mut gb = Gameboy::new();
//         let size = 255;
//         let mut rom: Vec<u8> = Vec::with_capacity(size as usize);
//         for i in 0..size {
//             rom.push(0);
//         }

//         gb.load(rom);
//         screen::render(gb);

//         gb.frame();

//         assert_eq!(gb.image(), 2);
//     }
// }
