mod cli;
mod config;
mod memo;
mod obsidian;

use clap::Parser;

use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("Hello, world!");
}
