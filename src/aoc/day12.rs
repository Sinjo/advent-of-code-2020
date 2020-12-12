#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    amount: isize
}

pub fn day12a(inputs: &[String]) -> anyhow::Result<String> {
    let instructions: Vec<_> = parse_input(inputs);

    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;
    let mut direction = 0;

    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::North => {
                north += instruction.amount;
            },
            Direction::South => {
                south += instruction.amount;
            },
            Direction::East => {
                east += instruction.amount;
            },
            Direction::West => {
                west += instruction.amount;
            },
            Direction::Left => {
                direction = modulo(direction - instruction.amount, 360);
            },
            Direction::Right => {
                direction = modulo(direction + instruction.amount, 360);
            },
            Direction::Forward => {
                match direction {
                    0 => {
                        east += instruction.amount;
                    },
                    90 => {
                        south += instruction.amount;
                    },
                    180 => {
                        west += instruction.amount;
                    },
                    270 => {
                        north += instruction.amount;
                    },
                    other => panic!("current bearing invalid: {}", other)
                }
            }
        }
    }

    let north_south = (north - south).abs();
    let east_west = (east - west).abs();

    let answer = north_south + east_west;

    Ok(answer.to_string())
}

fn parse_input(inputs: &[String]) -> Vec<Instruction> {
    let parsed = inputs.iter().map(|i| {
        let (direction_string, amount_string) = i.split_at(1);

        let direction = match direction_string {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            "L" => Direction::Left,
            "R" => Direction::Right,
            "F" => Direction::Forward,
            other => panic!("unrecognised direction: {}", other)
        };
        let amount = amount_string.parse().unwrap();

        Instruction{
            direction: direction,
            amount: amount
        }
    }).collect();

    parsed
}

fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}
