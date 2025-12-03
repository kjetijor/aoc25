use std::env::args;
use std::fs;
use aoc_25::dial::Dial;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    let moves = fs::read_to_string(&args[1])
        .expect("Failed to read input file").split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    let mut dial = Dial::new(100, 50);
    let mut zeroes = 0;
    for m in &moves {
        dial.do_move(m).unwrap();
        if dial.position == 0 {
            zeroes += 1;
        }
    }
    println!("zeroes is {}", zeroes);
    let mut dial = Dial::new(100, 50);
    for m in &moves {
        dial.do_move(m).unwrap();
    }
    println!("final position is {}, zero hits {}", dial.position, dial.zero_hits);
}