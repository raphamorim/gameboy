extern crate LR35902;
#[allow(non_snake_case)]
extern crate clap;

use clap::Parser;
use std::fs::File;
use std::io::Read;

use LR35902::gb;

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

fn main() {
    let args = Args::parse();
    let rom = get_rom_from_filepath(&args.filepath);

    let mut gb = gb::Gb::new();

    gb.load(rom.unwrap());
}
