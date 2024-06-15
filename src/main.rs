mod engine;
mod file_gen;

use clap::Parser;
use engine::engine_main;
use file_gen::gen_main;
use std::collections::{BTreeMap, HashMap};
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let result = if args.name == "gen" {
        gen_main(args)
    } else {
        engine_main(args)
    };

    let mut m: BTreeMap<&'static str, (u64, u64, usize)> = BTreeMap::new();
    unsafe {
        read_into(&mut m, 0, PROFILE_RECORDS.values.len(), 0);
    }

    for (name, (time, n, depth)) in m {
        let dash = "-".repeat(depth);
        let spaces = " ".repeat(depth);
        println!("{}{}", dash, name);
        println!("{}  total: {}", spaces, time);
        println!("{}  count: {}", spaces, n);
        println!("{}  avg: {}", spaces, time / n);
    }

    result
}

unsafe fn read_into(
    map: &mut BTreeMap<&'static str, (u64, u64, usize)>,
    i: usize,
    k: usize,
    depth: usize,
) -> usize {
    let mut index = i;
    loop {
        if index == k {
            return index;
        }
        let mut name: &'static str = "";
        let mut start: u64 = 0;
        let mut end: u64 = 0;
        if match PROFILE_RECORDS.values[index] {
            ProfPoint::Open(open, n) => {
                name = n;
                start = open;
                false
            }
            ProfPoint::Close(_) => true,
        } {
            return index;
        }
        map.entry(name).or_insert((0, 0, depth));
        index += 1;

        if match PROFILE_RECORDS.values[index] {
            ProfPoint::Open(_, _) => {
                index = read_into(map, index, k, depth + 1);
                true
            }
            ProfPoint::Close(n) => {
                end = n;
                false
            }
        } {
            match PROFILE_RECORDS.values[index] {
                ProfPoint::Open(_, _) => {
                    panic!("no good")
                }
                ProfPoint::Close(n) => {
                    end = n;
                }
            };
        }

        index += 1;
        let delta = end - start;
        map.entry(name).and_modify(|e| {
            e.0 += delta;
            e.1 += 1;
        });
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

pub struct ProfRecord {
    values: Vec<ProfPoint>,
}

pub enum ProfPoint {
    Open(u64, &'static str),
    Close(u64),
}

pub static mut PROFILE_RECORDS: ProfRecord = ProfRecord { values: Vec::new() };
