#[allow(non_snake_case)]
extern crate lr35902;
extern crate clap;

use clap::Parser;
use std::fs::File;
use std::io::Read;

use lr35902::{ gameboy, renderer::render };

#[derive(Parser, Debug)]
struct Args {
    /// Filepath
    #[clap(default_value_t = String::from(""))]
    filepath: String,

    /// Browser
    #[clap(default_value_t = true, short, long)]
    browser: bool,

    /// Desktop
    #[clap(short, long)]
    desktop: bool,

    /// Enable Audio
    #[clap(short, long)]
    audio: bool,

    /// Scale screen
    #[clap(default_value_t = 1)]
    scale: u8,
}

fn get_rom_from_filepath(filepath: &String) -> Result<Vec<u8>, String> {
    let mut rom = Vec::new();
    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
        Ok(..) => {}
        Err(e) => return Err(format!("failed to read {}: {}", filepath, e)),
    };

    Ok(rom)
}

fn add_yellow_color(text: &str) -> String {
    format!("{}{}{}", "\x1b[93m", text, "\x1b[0m")
}

fn main() {
    let args = Args::parse();
    let rom = get_rom_from_filepath(&args.filepath);
    let cmd = add_yellow_color("[LR35902]");

    let mut gb = gameboy::Gameboy::new();

    if &args.filepath == "" {
        println!("{} Please provide a rom file", cmd);        
        return;
    }

    println!("{} Gameboy loading... {:?}", cmd, &args.filepath);

    gb.load(rom.unwrap());

    render(gb);
}

// mod gl {
//     include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// }
