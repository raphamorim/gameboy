#![allow(non_snake_case)]

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

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
