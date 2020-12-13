pub fn day13a(inputs: &[String]) -> anyhow::Result<String> {
    let earliest_departure: usize = inputs[0].parse().unwrap();
    let buses: Vec<usize> = inputs[1].split(",").filter_map(|bus| {
        match bus {
            "x" => None,
            id => Some(id.parse().unwrap())
        }
    }).collect();

    let mut earliest_per_bus: Vec<_> = buses.iter().map(|bus| {
        (bus, ((1 + earliest_departure / bus) * bus) - earliest_departure)
    }).collect();
    earliest_per_bus.sort_by(|(_, waiting_a), (_, waiting_b)| waiting_a.cmp(waiting_b));

    let (id, time_waiting) = earliest_per_bus[0];
    let answer = id * time_waiting;

    Ok(answer.to_string())
}

// Problem:
//
// Find the smallest timestamp such that for each element in (offset, bus_id)
// 
//   (timestamp + offset) % bus_id = 0
//
//   (0,7), (1, 13)                     //, (2, 17)
//
//   77 is smallest timestamp
//   77 % 7 = 0
//   (77 + 1) % 13 = 0
//
// or
//
// Find the smallest timestamp such that for each element in (bus_id, (bus_id - offset) % bus_id)
//
//   timestamp % first_in_part = second_in_pair 
pub fn day13b(inputs: &[String]) -> anyhow::Result<String> {
    let buses: Vec<(i128, i128)> = inputs[1].split(",").enumerate().filter_map(|(index,bus)| {
        match bus {
            "x" => None,
            id => {
                let parsed: i128 = id.parse().unwrap();
                // Some((parsed, modulo(parsed - index as i128, parsed)))
                Some((parsed, -(index as i128)))
            }
        }
    }).collect();
    // buses.sort_by(|(id_a, _), (id_b, _)| id_b.cmp(id_a));

    // let test = extended_euclidian(7, 13);
    // println!("{:#?}", test);

    // let test2 = chinese_remainder_theorem((7, 0), (13, -1));
    // println!("{:#?}", test2);

    // println!("{:#?}", buses);
    
    let head = buses[0];
    let (_, tail) = buses.split_at(1);

    let (final_mod, final_product) = tail.iter().fold(head, |acc, next| {
        chinese_remainder_theorem(acc, *next)
    });

    let answer = modulo(final_product, final_mod);

    println!("{} {}", final_mod, final_product);
    println!("{}", answer);

    Ok("whatever".to_string())
}

fn chinese_remainder_theorem(input1: (i128, i128), input2: (i128, i128)) -> (i128, i128) {
    let (mod1, remainder1) = input1;
    let (mod2, remainder2) = input2;
    let (_, x, y) = extended_euclidian(mod1, mod2);
    let new_remainder = remainder1 * y * mod2 + remainder2 * x * mod1;

    (mod1 * mod2, new_remainder)
}

fn extended_euclidian(a: i128, b: i128) -> (i128, i128, i128) {
    let mut remainder = a;
    let mut x = 1;
    let mut y = 0;
    let mut remainder_next = b;
    let mut x_next = 0;
    let mut y_next = 1;

    loop {
        if remainder_next == 0 {
            return (remainder, x, y);
        }

        // calculate new values
        let quotient = remainder / remainder_next;
        let remainder_new = remainder - quotient * remainder_next;
        let x_new = x - quotient * x_next;
        let y_new = y - quotient * y_next;

        // shift values along
        remainder = remainder_next;
        x = x_next;
        y = y_next;
        remainder_next = remainder_new;
        x_next = x_new;
        y_next = y_new;
    }
}

fn modulo(a: i128, b: i128) -> i128 {
    ((a % b) + b) % b
}
