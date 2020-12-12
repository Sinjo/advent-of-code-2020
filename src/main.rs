use std::collections::HashMap;
use std::time::Instant;
use std::fs;
use std::process;
use libaoc::aoc;
use clap::{Arg, App};


fn main() {
    let mut solutions: HashMap<&str, fn(&[String]) -> anyhow::Result<String>> = HashMap::new();
    solutions.insert("1a", aoc::day1::day1a);
    solutions.insert("1b", aoc::day1::day1b);
    solutions.insert("2a", aoc::day2::day2a);
    solutions.insert("2b", aoc::day2::day2b);
    solutions.insert("3a", aoc::day3::day3a);
    solutions.insert("3b", aoc::day3::day3b);
    solutions.insert("4a", aoc::day4::day4a);
    solutions.insert("4b", aoc::day4::day4b);
    solutions.insert("5a", aoc::day5::day5a);
    solutions.insert("5b", aoc::day5::day5b);
    solutions.insert("6a", aoc::day6::day6a);
    solutions.insert("6b", aoc::day6::day6b);
    solutions.insert("7a", aoc::day7::day7a);
    solutions.insert("7b", aoc::day7::day7b);
    solutions.insert("8a", aoc::day8::day8a);
    solutions.insert("8b", aoc::day8::day8b);
    solutions.insert("9a", aoc::day9::day9a);
    solutions.insert("9b", aoc::day9::day9b);
    solutions.insert("10a", aoc::day10::day10a);
    solutions.insert("10b", aoc::day10::day10b);
    solutions.insert("11a", aoc::day11::day11a);
    solutions.insert("11b", aoc::day11::day11b);

    let matches = App::new("aoc2020")
        .version("0.1.0")
        .author("Chris Sinjakli <chris@sinjakli.co.uk>")
        .about("Solutions to Advent of Code 2020")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("Runs the solution for a specified day")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("puzzle")
            .short("p")
            .long("puzzle")
            .value_name("PUZZLE")
            .help("Runs the first or second puzzle for the day, denoted by 'a' or 'b'")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input-file")
            .short("f")
            .long("input-file")
            .value_name("FILE")
            .help("Runs the solution with the specified input file")
            .takes_value(true)
            .required(true))
        .get_matches();

    let day_str = match matches.value_of("day") {
        Some(s) => s,
        None => {
            println!("Key 'day' not found in matches (code broke, fix the code)");
            process::exit(1);
        }
    };
    let _day: u32 = match day_str.parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid day specified: {}", day_str);
            process::exit(1);
        },
    };

    let puzzle = match matches.value_of("puzzle") {
        Some(s) => s,
        None => {
            println!("Key 'puzzle' not found in matches (code broke, fix the code)");
            process::exit(1);
        }
    };

    let input_file = match matches.value_of("input-file") {
        Some(s) => s,
        None => {
            println!("Key 'input-file' not found in matches (code broke, fix the code)");
            process::exit(1);
        }
    };

    let input = match fs::read_to_string(input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading input file '{}': {}", input_file, e);
            process::exit(1);
        }
    };

    let input_lines: Vec<String> = input.lines().map(str::to_string).collect();
    let qualified_puzzle = [day_str, puzzle].concat();

    let start_time = Instant::now();
    let result = solutions[qualified_puzzle.as_str()](&input_lines);
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    match result {
        Ok(r) => {
            println!("Solution to puzzle {} is: {}", qualified_puzzle, r);
            println!("Time taken: {:#?}", duration)
        },
        Err(e) => println!("Error running puzzle {}: {}", qualified_puzzle, e)
    }
}
