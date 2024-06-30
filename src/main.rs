mod engine;
mod file_gen;

use clap::Parser;
use engine::engine_main;
use file_gen::gen_main;
use std::io::Result;
use std::time::{Instant};

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
    unsafe {
        println!("clock: {:.2}mhz", cpu_estimate(100) as f64 / 1_000_000f64)
    }

    result
}

/// returns cpu clock estimate in hz.
///  * `ms_to_wait` - millis to wait to estimate cpu clock.
unsafe fn cpu_estimate(ms_to_wait: u64) -> u64 {
    if cfg!(target_os = "linux") {
        use nix::sys::time::{TimeValLike};
        let os_freq = 1_000_000_000u64; // can be calculated with nix::time::clock_getres
        let ms_per_ns = 1_000_000u64; // can be calculated with nix::time::clock_getres

        let measure_time = ms_to_wait as i64 * ms_per_ns as i64;
        let mut elapsed_os = 0i64;

        let pre = core::arch::x86_64::_rdtsc();
        let start = nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC).unwrap();
        while elapsed_os < measure_time {
            let cur = nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC).unwrap();
            elapsed_os = (cur - start).num_nanoseconds();
        }
        let post = core::arch::x86_64::_rdtsc();
        let elapsed_clock = post-pre;

        elapsed_clock * os_freq / elapsed_os as u64
    } else {
        panic!("not implemented for other OSes")
    }
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
