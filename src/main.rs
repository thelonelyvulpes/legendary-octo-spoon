mod engine;
mod file_gen;

use clap::Parser;
use engine::engine_main;
use file_gen::gen_main;
use std::io::Result;
use std::time::{Duration, Instant};

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
    unsafe { println!("clock: {:.2}mhz", cpu_estimate(100) as f64 / 1_000_000f64) }

    result
}

/// returns cpu clock estimate in hz.
///  * `ms_to_wait` - millis to wait to estimate cpu clock.
unsafe fn cpu_estimate(ms_to_wait: u64) -> u64 {
    let pre = core::arch::x86_64::_rdtsc();
    let dur = Duration::from_millis(ms_to_wait);
    let start = Instant::now();
    while Instant::now().duration_since(start) < dur {}
    let post = core::arch::x86_64::_rdtsc();
    let elapsed_clock = post - pre;

    (1000u64 / ms_to_wait) * elapsed_clock
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

// move to time-macro?
pub struct ProfRecord {
    values: Vec<ProfPoint>,
}

pub enum ProfPoint {
    Open(u64, &'static str),
    Close(u64),
    Data(u64),
}

pub static mut PROFILE_RECORDS: ProfRecord = ProfRecord { values: Vec::new() };
