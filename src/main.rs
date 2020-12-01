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
    let _better_day: u32 = match day_str.parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid day specified: {}", day_str);
            process::exit(1);
        },
    };

    println!("AoC module returns: {}", aoc::do_aoc_stuff());
}
