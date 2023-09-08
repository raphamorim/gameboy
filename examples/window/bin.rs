extern crate gameboy;

use gameboy::gameboy::{load_rom, Gameboy, RenderMode::Desktop};

fn main() {
    // let gb = Gameboy::new("./../../tests/cpu_instrs/cpu_instrs.gb");
    if let Ok((data, filepath)) = load_rom("./../pokemon-blue.gb") {
    // if let Ok((data, filepath)) = load_rom("./../../tests/dmg_sound/dmg_sound.gb") {
        // na web:
        // let gb = Gameboy::new(data, None);
        let gb = Gameboy::new(data, Some(filepath));
        gb.render(Desktop);
    } else {
        println!("error loading rom");
    }
}
