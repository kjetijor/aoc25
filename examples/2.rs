use aoc_25::invalidids::{iter_ranges, naive_invalid_ids};
use std::fs::File;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }
    let mut res = 0u64;
    let file = File::open(&args[1]).expect("Failed to open input file");
    for r in iter_ranges(file) {
        let r = r.expect("Failed to parse range");
        res += naive_invalid_ids(&r).into_iter().sum::<u64>();
    }
    println!("Sum of invalid IDs: {}", res);
}
