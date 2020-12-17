use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point4d {
    x: isize,
    y: isize,
    z: isize,
    w: isize
}

pub fn day17a(inputs: &[String]) -> anyhow::Result<String> {
    let mut alive = parse_game(inputs);
    let mut alive_next = HashSet::new();

    // For each point in alive
    //   - Generate neighbours
    //   - See how many neighbours alive (set itersection with alive)
    //   - If 2 or 3 -> insert into alive_next
    //   - For each of those neighbours
    //     - Generate neighbours
    //     - See how many neghbours alive (set intersection with alive)
    //     - If 3 -> insert into alive_next

    for _ in 0..6 {
        for point in alive.iter() {
            let neighbours = generate_neighbours(point);
            let alive_neighbours: HashSet<_> = alive.intersection(&neighbours).collect();

            if alive_neighbours.len() == 2 || alive_neighbours.len() == 3 {
                alive_next.insert(point.clone());
            }

            for neighbour in neighbours.iter() {
                let neighbours = generate_neighbours(neighbour);
                let alive_neighbours: HashSet<_> = alive.intersection(&neighbours).collect();

                if alive_neighbours.len() == 3 {
                    alive_next.insert(neighbour.clone());
                }
            }
        }

        alive = alive_next;
        alive_next = HashSet::new();
    }

    let answer = alive.len();

    Ok(answer.to_string())
}

pub fn day17b(inputs: &[String]) -> anyhow::Result<String> {
    let mut alive = parse_game4d(inputs);
    let mut alive_next = HashSet::new();

    for _ in 0..6 {
        for point in alive.iter() {
            let neighbours = generate_neighbours4d(point);
            let alive_neighbours: HashSet<_> = alive.intersection(&neighbours).collect();

            if alive_neighbours.len() == 2 || alive_neighbours.len() == 3 {
                alive_next.insert(point.clone());
            }

            for neighbour in neighbours.iter() {
                let neighbours = generate_neighbours4d(neighbour);
                let alive_neighbours: HashSet<_> = alive.intersection(&neighbours).collect();

                if alive_neighbours.len() == 3 {
                    alive_next.insert(neighbour.clone());
                }
            }
        }

        alive = alive_next;
        alive_next = HashSet::new();
    }

    let answer = alive.len();

    Ok(answer.to_string())
}

fn parse_game(inputs: &[String]) -> HashSet<Point> {
    let as_chars: Vec<Vec<_>> = inputs.iter().map(|s| s.chars().collect()).collect();

    let mut alive_at_start = HashSet::new();

    for y in 0..as_chars.len() {
        for x in 0..as_chars[y].len() {
            match as_chars[y][x] {
                '#' => { alive_at_start.insert(Point{x: x as isize, y: y as isize, z:0}); },
                '.' => (),
                other => panic!("unrecognised input: {}", other)
            }
        }
    }

    alive_at_start
}

fn parse_game4d(inputs: &[String]) -> HashSet<Point4d> {
    let as_chars: Vec<Vec<_>> = inputs.iter().map(|s| s.chars().collect()).collect();

    let mut alive_at_start = HashSet::new();

    for y in 0..as_chars.len() {
        for x in 0..as_chars[y].len() {
            match as_chars[y][x] {
                '#' => { alive_at_start.insert(Point4d{x: x as isize, y: y as isize, z:0, w:0}); },
                '.' => (),
                other => panic!("unrecognised input: {}", other)
            }
        }
    }

    alive_at_start
}

fn generate_neighbours(point: &Point) -> HashSet<Point> {
    let mut neighbours = HashSet::new();

    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            for z in (point.z - 1)..=(point.z + 1) {
                if x == point.x && y == point.y && z == point.z {
                    continue;
                }
                neighbours.insert(Point{x, y, z});
            }
        }
    }

    neighbours
}

fn generate_neighbours4d(point: &Point4d) -> HashSet<Point4d> {
    let mut neighbours = HashSet::new();

    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            for z in (point.z - 1)..=(point.z + 1) {
                for w in (point.w - 1)..=(point.w + 1) {
                    if x == point.x && y == point.y && z == point.z && w == point.w {
                        continue;
                    }
                    neighbours.insert(Point4d{x, y, z, w});
                }
            }
        }
    }

    neighbours
}
