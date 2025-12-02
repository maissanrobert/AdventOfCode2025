use clap::Parser;
use std::collections::HashMap;

use advent_of_code_2025::*;

#[derive(Parser)]
#[command(name = "AdventOfCode2025")]
#[command(about = "Rust solutions for AdventOfCode2025 2025", long_about = None)]
struct Args {
    //Name of the day to run
    #[arg(short, long)]
    day: String,

    //Path to the input file
    #[arg(short, long)]
    input: Option<String>,

    //Part to run
    #[arg(short, long)]
    part: Option<u8>,
}

fn main() {
    let days = HashMap::from(
        [
            ("day01", day01::run as fn(Option<String>, Option<u8>)),
            ("1", day01::run as fn(Option<String>, Option<u8>)),
            ("day02", day02::run as fn(Option<String>, Option<u8>)),
            ("2", day02::run as fn(Option<String>, Option<u8>)),
            //("day03", day03::run as fn(Option<String>, Option<u8>)),
            //("3", day03::run as fn(Option<String>, Option<u8>)),
        ],
    );

    let args = Args::parse();

    let day = args.day;

    match days.get(day.as_str()) {
        Some(run) => run(args.input, args.part),
        None => {
            println!("Unknown day: {}", day);
            println!();
            print_usage();
        }
    }
}

fn print_usage() {
    println!("AdventOfCode2025 2025 - Rust Solutions");
    println!();
    println!("Usage: cargo run -- <day>");
    println!();
    println!("Examples:");
    println!("  cargo run -- day01");
    println!("  cargo run -- 1");
    println!("  cargo run -- day02");
    println!();
}
