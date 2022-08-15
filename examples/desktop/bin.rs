extern crate lr35902;
extern crate clap;

use lr35902::{ gameboy::Gameboy, screen::desktop };

fn main() {
    let mut gb = Gameboy::new();
    let rom: Vec<u8> = gb.read_rom_by_filepath("./sample-rom.gb");
    // let rom: Vec<u8> = gb.read_rom_by_filepath("./cpu_instrs.gb");
    gb.load(rom);
    desktop::render(gb);
}
