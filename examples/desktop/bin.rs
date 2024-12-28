extern crate gameboy;

use gameboy::gameboy::{load_rom, Gameboy, RenderMode::Desktop};

fn main() {
    // TODO: Allow receive path by arguments
    // let gb = Gameboy::new("./../../tests/cpu_instrs/cpu_instrs.gb");
    if let Ok((data, filepath)) = load_rom("./../the-machine.gb") {
    // if let Ok((data, filepath)) = load_rom("./../bakery.gb") {
        let gb = Gameboy::new(data, Some(filepath));
        gb.render(Desktop);
    } else {
        panic!("error loading rom");
    }
}
