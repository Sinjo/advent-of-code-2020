use num::BigInt;

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
//   (0,7), (1, 13)
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
    let buses: Vec<(BigInt, BigInt)> = inputs[1].split(",").enumerate().filter_map(|(index,bus)| {
        match bus {
            "x" => None,
            id => {
                let parsed: BigInt = id.parse().unwrap();
                // This and the non-commented line below are equivalent because the calculations we
                // do later are all modulo some number. That means we can either start with the
                // offset of the bus modulo the frequency (e.g. 13 - 1 % 13 = 12) or the offset itself
                // (-1).
                //
                // Some((parsed.clone(), modulo(&(parsed.clone() - BigInt::from(index)), &parsed.clone())))
                Some((parsed, -(BigInt::from(index))))
            }
        }
    }).collect();
    
    let head = buses[0].clone();
    let (_, tail) = &buses.split_at(1);

    let (final_mod, final_product) = tail.iter().cloned().fold(head, |acc, next| {
        chinese_remainder_theorem(acc, next)
    });

    let answer = modulo(&final_product, &final_mod);

    Ok(answer.to_string())
}

fn chinese_remainder_theorem(input1: (BigInt, BigInt), input2: (BigInt, BigInt)) -> (BigInt, BigInt) {
    let (mod1, remainder1) = input1;
    let (mod2, remainder2) = input2;
    let (_, x, y) = extended_euclidian(mod1.clone(), mod2.clone());
    let new_remainder = remainder1 * y * &mod2 + remainder2 * x * &mod1;

    (mod1 * mod2, new_remainder)
}

fn extended_euclidian(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    let mut remainder = a;
    let mut x = BigInt::from(1);
    let mut y = BigInt::from(0);
    let mut remainder_next = b;
    let mut x_next = BigInt::from(0);
    let mut y_next = BigInt::from(1);

    loop {
        if remainder_next == BigInt::from(0) {
            return (remainder, x, y);
        }

        // calculate new values
        let quotient = &remainder / &remainder_next;
        let remainder_new = remainder - &quotient * &remainder_next;
        let x_new = x - &quotient * &x_next;
        let y_new = y - &quotient * &y_next;

        // shift values along
        remainder = remainder_next;
        x = x_next;
        y = y_next;
        remainder_next = remainder_new;
        x_next = x_new;
        y_next = y_new;
    }
}

fn modulo(a: &BigInt, b: &BigInt) -> BigInt {
    ((a % b) + b) % b
}
