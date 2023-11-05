use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufWriter, Result};
use std::io::{LineWriter, Write};
use rand::prelude::ThreadRng;
use crate::{Args, haversine};


pub fn gen_main(args: Args) -> Result<()>{
    if args.count == 0 {
        return Ok(());
    }

    let main_file_name = format!("{}.json", args.file_name);
    let bin_file_name = format!("{}.bin", args.file_name);

    let file = File::create(main_file_name)?;
    let mut lw = LineWriter::new(file);
    let bin = File::create(bin_file_name)?;
    let mut bw = BufWriter::new(bin);

    lw.write_all(b"{ \"pairs\": [\n")?;

    let mut sum = 0f64;

    let mut random = thread_rng();
    for i in 1..=args.count {
        let (x0, x1, y0, y1) = gen_vals(&mut random);

        lw.write_fmt(format_args!(
            "{{\"x0\": {},\"x1\": {}, \"y0\": {},\"y1\": {}}}{}\n",
            x0, x1, y0, y1, if i == args.count  { "" } else { "," }
        ))?;

        let res = haversine(x0, x1, y0, y1);
        sum += res;
        let val = res.to_le_bytes();
        bw.write_all(&val)?;
    }
    lw.write_all(b"]}")?;

    println!("avg: {}", sum / args.count as f64);
    Ok(())
}


fn gen_vals(random: &mut ThreadRng) -> (f64, f64, f64, f64) {
    let x0 = random.gen_range(-180.0..=180.0);
    let x1 = random.gen_range(-180.0..=180.0);
    let y0 = random.gen_range(-90.0..=90.0);
    let y1 = random.gen_range(-90.0..=90.0);
    (x0, x1, y0, y1)
}