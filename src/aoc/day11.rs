use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SeatState {
    Floor,
    Empty,
    Occupied
}

pub fn day11a(inputs: &[String]) -> anyhow::Result<String> {
    let chars: Vec<Vec<_>> = inputs.iter().map(|s| s.chars().collect()).collect();

    let initial_state: BTreeMap<(usize, usize), SeatState> = parse_state(&chars);
    let neighbours: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> = generate_neighbours(&initial_state);

    let mut last_state: BTreeMap<(usize, usize), SeatState> = initial_state.clone();
    let mut next_state: BTreeMap<(usize, usize), SeatState> = BTreeMap::new();

    loop {
        for ((x,y), seat_state) in last_state.iter() {
            match seat_state {
                SeatState::Floor => { next_state.insert((*x,*y), *seat_state); },
                SeatState::Empty => {
                    let seat_neighbours = neighbours.get(&(*x,*y)).unwrap();

                    let occupied_neighbours: Vec<_> = seat_neighbours.iter().filter(|n| {
                        *last_state.get(&n).unwrap() == SeatState::Occupied
                    }).collect();

                    if occupied_neighbours.len() == 0 {
                        next_state.insert((*x,*y), SeatState::Occupied);
                    } else {
                        next_state.insert((*x,*y), *seat_state);
                    }
                },
                SeatState::Occupied => {
                    let seat_neighbours = neighbours.get(&(*x,*y)).unwrap();

                    let occupied_neighbours: Vec<_> = seat_neighbours.iter().filter(|n| {
                        *last_state.get(&n).unwrap() == SeatState::Occupied
                    }).collect();

                    if occupied_neighbours.len() >= 4 {
                        next_state.insert((*x,*y), SeatState::Empty);
                    } else {
                        next_state.insert((*x,*y), *seat_state);
                    }
                }
            }
        }

        if last_state == next_state {
            let occupied_seats = last_state.iter().filter(|(_, seat_state)| {
                seat_state == &&SeatState::Occupied
            }).collect::<Vec<_>>().len();

            return Ok(occupied_seats.to_string());
        } else {
            last_state = next_state;
            next_state = BTreeMap::new();
        }
    }
}

fn parse_state(input: &Vec<Vec<char>>) -> BTreeMap<(usize, usize), SeatState> {
    // x
    let num_columns = input[0].len();
    // y
    let num_rows = input.len();

    let mut state: BTreeMap<(usize, usize), SeatState> = BTreeMap::new();

    for x in 0..num_columns {
        for y in 0..num_rows {
            let seat_state = match input[y][x] {
                '.' => SeatState::Floor,
                'L' => SeatState::Empty,
                '#' => SeatState::Occupied,
                c => panic!("unrecognised state: {}", c)
            };

            state.insert((x, y), seat_state);
        }
    }

    state
}

fn generate_neighbours(state: &BTreeMap<(usize, usize), SeatState>) -> BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> {
    let ((max_x, max_y), _) = state.iter().next_back().unwrap();
    let mut neighbours: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> = BTreeMap::new();

    for (x,y) in state.keys().copied() {
        // y tho
        let generated_neighbours = [
            (x - min(1, x), y - min(1, y)), (x,  y - min(1, y)), (x+1, y - min(1, y)),
            (x - min(1, x), y ),                                 (x+1, y  ),
            (x - min(1, x), y+1),           (x,  y+1),           (x+1, y+1)
        ];

        let valid = generated_neighbours.iter().filter(|(gen_x,gen_y)| {
            gen_x <= max_x &&
                gen_y <= max_y &&
                (x, y) != (*gen_x, *gen_y)
        }).copied().collect();

        let as_set = BTreeSet::from(valid);

        neighbours.insert((x,y), as_set);
    }

    neighbours
}
