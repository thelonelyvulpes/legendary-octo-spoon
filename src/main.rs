mod engine;
mod file_gen;
use clap::Parser;

use engine::engine_main;
use file_gen::gen_main;
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    if args.name == "gen" {
        return gen_main(args);
    }
    return engine_main(args);
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
pub struct Args {
    name: String,
    /// Number of times to greet
    count: u32,
    #[arg(short, long, default_value = "test")]
    file_name: String,
}
