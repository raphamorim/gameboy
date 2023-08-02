extern crate lr35902;

use lr35902::gameboy::{load_rom, Gameboy, RenderMode::Desktop};

fn main() {
    // let gb = Gameboy::new("./../../tests/cpu_instrs/cpu_instrs.gb");
    if let Ok((data, filepath)) = load_rom("./../pokemon-blue.gb") {
        let gb = Gameboy::new(data, filepath);
        gb.render(Desktop);
    } else {
        println!("error loading rom");
    }
    
    // match gb.load_rom("./pokemon-blue.gb") {
    // match gb.load_rom("./sample-rom.gb") {
    // match gb.load_rom("./../../tests/cpu_instrs/cpu_instrs.gb") {
    //     Ok(..) => {
    //     }
    //     Err(err) => {
    //         println!("{:?}", err);
    //     }
    // }
}
