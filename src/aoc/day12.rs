use std::iter;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Matrix {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize
}

pub fn day12a(inputs: &[String]) -> anyhow::Result<String> {
    let instructions: Vec<_> = parse_input(inputs);

    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;
    let mut bearing = 0;

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
                bearing = modulo(bearing - instruction.amount, 360);
            },
            Direction::Right => {
                bearing = modulo(bearing + instruction.amount, 360);
            },
            Direction::Forward => {
                match bearing {
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

pub fn day12b(inputs: &[String]) -> anyhow::Result<String> {
    let instructions: Vec<_> = parse_input(inputs);

    let mut waypoint = Position{x: 10, y: 1};
    let mut position = Position{x: 0, y: 0};

    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::North => {
                waypoint.y += instruction.amount
            },
            Direction::South => {
                waypoint.y -= instruction.amount
            },
            Direction::East => {
                waypoint.x += instruction.amount
            },
            Direction::West => {
                waypoint.x -= instruction.amount
            },
            Direction::Left => {
                // x = -y, y = x
                // 2,1 -> -1,2
                // -1,2 -> -2, -1
                // -2, -1 -> 1, -2
                // 1, -2 -> 2, 1
                let matrix = generate_anticlockwise_rotation_matrix(instruction.amount);
                let new_waypoint = Position {
                    x: matrix.x1 * waypoint.x + matrix.x2 * waypoint.y,
                    y: matrix.y1 * waypoint.x + matrix.y2 * waypoint.y
                };

                waypoint = new_waypoint
            },
            Direction::Right => {
                // x = y, y = -x
                // 2,1 -> 1, -2
                // 1,-2 -> -2, -1
                // -2,-1 -> -1, 2
                // -1,2 -> 2,1
                let matrix = generate_clockwise_rotation_matrix(instruction.amount);
                let new_waypoint = Position {
                    x: matrix.x1 * waypoint.x + matrix.x2 * waypoint.y,
                    y: matrix.y1 * waypoint.x + matrix.y2 * waypoint.y
                };

                waypoint = new_waypoint
            },
            Direction::Forward => {
                position.x += waypoint.x * instruction.amount;
                position.y += waypoint.y * instruction.amount;
            }
        }
    }

    let answer = position.x.abs() + position.y.abs();

    Ok(answer.to_string())
}

fn generate_anticlockwise_rotation_matrix(degrees: isize) -> Matrix {
    let left_90 = Matrix{x1: 0, x2: -1, y1: 1, y2: 0};
    let num_rotations = degrees / 90;

    let rotation = rotate_n_times(left_90, num_rotations as usize);

    rotation
}

fn generate_clockwise_rotation_matrix(degrees: isize) -> Matrix {
    let right_90 = Matrix{x1: 0, x2: 1, y1: -1, y2: 0};
    let num_rotations = degrees / 90;

    let rotation = rotate_n_times(right_90, num_rotations as usize);

    rotation
}

fn rotate_n_times(matrix: Matrix, n: usize) -> Matrix {
    let identity = Matrix{x1: 1, x2: 0, y1: 0, y2: 1};

    let matrices = iter::repeat(matrix).take(n);
    let generated = matrices.fold(identity, |acc, m| {
        multiply_matrices(m, acc)
    });

    generated
}

fn multiply_matrices(left: Matrix, right: Matrix) -> Matrix {
    Matrix{
        x1: left.x1 * right.x1 + left.x2 * right.y1,
        x2: left.x1 * right.x2 + left.x2 * right.y2,
        y1: left.y1 * right.x1 + left.y2 * right.y1,
        y2: left.y1 * right.x2 + left.y2 * right.y2
    }
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
