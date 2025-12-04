use aoc_25::invalidids::{iter_ranges, naive_invalid_id, naive_invalid_id_pt2, naive_invalid_ids};
use std::fs::File;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }
    let mut res = 0u64;
    let mut res_pt2 = 0u64;
    let file = File::open(&args[1]).expect("Failed to open input file");
    for r in iter_ranges(file) {
        let r = r.expect("Failed to parse range");
        res += naive_invalid_ids(&r, naive_invalid_id).into_iter().sum::<u64>();
        res_pt2 += naive_invalid_ids(&r, naive_invalid_id_pt2).into_iter().sum::<u64>();
    }
    println!("Sum of invalid IDs: {}, Sum of invalid IDs using second method {}", res, res_pt2);
}
