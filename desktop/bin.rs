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

    /// Enable Audio
    #[clap(short, long)]
    audio: bool,

    /// Scale screen
    #[clap(default_value_t = 1)]
    scale: u8,
}

fn add_yellow_color(text: &str) -> String {
    format!("{}{}{}", "\x1b[93m", text, "\x1b[0m")
}

fn main() {
    let args = Args::parse();
    let cmd = add_yellow_color("[LR35902]");
    let mut gb = gameboy::Gameboy::new();
    // gb.set_scale(&args.scale);
    let filepath = &args.filepath;

    if &args.filepath == "" {
        println!("{} Please provide a rom file", cmd);        
        return;
    }

    println!("{} ROM Path: {:?}", cmd, filepath);

    let mut rom = Vec::new();
    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
        Ok(..) => {}
        Err(e) => {
            println!("failed to read {}: {}", filepath, e);
            return
        },
    };

    gb.load(rom);
    render(gb);
}
