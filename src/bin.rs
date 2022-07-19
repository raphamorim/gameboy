#![allow(non_snake_case)]

use std::fs;
use clap::Parser;

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
}

fn read_rom_from_path(rom_path: &String) -> Result<Vec<u8>, String> {
    match fs::read(rom_path) {
        Ok(rom) => Ok(rom),
        Err(_) => Err(format!("Could not open file {}", rom_path)),
    }
}

fn main() {
    let args = Args::parse();
    let rom = read_rom_from_path(&args.filepath);
    
    println!("{:?}", rom);
}
