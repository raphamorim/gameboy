extern crate lr35902;
extern crate clap;

use lr35902::{ gameboy, renderer::render };

fn main() {
    let mut gb = gameboy::Gameboy::new();
    let rom: Vec<u8> = gb.read_rom_by_filepath("./sample-rom.gb");
    
    // Default scale is 1 (160x144) 
    gb.set_scale(2).load(rom);
    render(gb);
}
