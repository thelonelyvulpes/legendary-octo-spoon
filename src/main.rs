mod engine;
mod file_gen;

use clap::Parser;
use engine::engine_main;
use file_gen::gen_main;
use std::io::Result;
use std::time::Instant;

fn main() -> Result<()> {
    let args = Args::parse();
    let start_wall_clock = Instant::now();
    let result = if args.name == "gen" {
        gen_main(args)
    } else {
        engine_main(args)
    };
    let end_wall_clock = Instant::now();

    println!(
        "wall: {}ms",
        end_wall_clock.duration_since(start_wall_clock).as_millis()
    );

    result
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

pub struct ProfRecord {
    values: Vec<ProfPoint>,
}

pub enum ProfPoint {
    Open(u64, &'static str),
    Close(u64),
    Data(u64),
}

pub static mut PROFILE_RECORDS: ProfRecord = ProfRecord { values: Vec::new() };
