extern crate lr35902;

use lr35902::gameboy::{Gameboy, RenderMode::Desktop};

fn main() {
    // let gb = Gameboy::new("./../../tests/cpu_instrs/cpu_instrs.gb");
    let gb = Gameboy::new("./pokemon-blue.gb");
    // match gb.load_rom("./pokemon-blue.gb") {
    // match gb.load_rom("./sample-rom.gb") {
    // match gb.load_rom("./../../tests/cpu_instrs/cpu_instrs.gb") {
    //     Ok(..) => {
     gb.render(Desktop);
    //     }
    //     Err(err) => {
    //         println!("{:?}", err);
    //     }
    // }
}
