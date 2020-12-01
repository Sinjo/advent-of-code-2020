use std::process;
use libaoc::aoc;
use clap::{Arg, App};

fn main() {
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

    let _puzzle = match matches.value_of("puzzle") {
        Some(s) => s,
        None => {
            println!("Key 'puzzle' not found in matches (code broke, fix the code)");
            process::exit(1);
        }
    };

    let _input_file = match matches.value_of("input-file") {
        Some(s) => s,
        None => {
            println!("Key 'input-file' not found in matches (code broke, fix the code)");
            process::exit(1);
        }
    };

    println!("AoC module returns: {}", aoc::do_aoc_stuff());
}
