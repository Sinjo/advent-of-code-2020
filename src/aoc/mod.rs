use std::collections::HashSet;

pub fn do_aoc_stuff() -> bool {
    true
}

pub fn day1a(inputs: &[String]) -> i32 {
    // TODO: Handle errors, don't just unwrap and hope for the best
    let amounts: Vec<i32> = inputs.into_iter().map(|x| x.parse().unwrap()).collect();

    let mut lookup = HashSet::new();
    for amount in amounts.iter() {
        let desired = 2020 - amount;

        if lookup.contains(&desired) {
            return amount * desired;
        }

        lookup.insert(amount);
    }

    // TODO: Maybe error instead of returning a garbage value
    0
}

pub fn day1b(inputs: &[String]) -> i32 {
    // TODO: Handle errors, don't just unwrap and hope for the best
    let amounts: Vec<i32> = inputs.into_iter().map(|x| x.parse().unwrap()).collect();

    let mut lookup = HashSet::new();
    for amount in amounts.iter() {
        lookup.insert(amount);
    }

    for i in amounts.iter() {
        for j in amounts.iter() {
            let desired = 2020 - i - j;

            if lookup.contains(&desired) {
                return i * j * desired;
            }
        }
    }

    // TODO: Maybe error instead of returning a garbage value
    0
}
