mod engine;
mod file_gen;
use clap::Parser;

use engine::{engine_main};
use file_gen::{gen_main};
use std::io::{Result};

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
    file_name: String
}

fn haversine(x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let d_lat = f64::to_degrees(y1 - y0);
    let d_lon = f64::to_degrees(x1 - x0);
    let lat1 = f64::to_degrees(y0);
    let lat2 = f64::to_degrees(y1);

    let a = ((d_lat / 2.0f64).sin().powi(2)) + lat1.cos() * lat2.cos() * ((d_lon / 2.0f64).sin().powi(2));
    let c = 2.0f64 * (a.sqrt().asin());
    6372.8f64 * c
}