use std::collections::HashSet;

pub fn do_aoc_stuff() -> bool {
    true
}

pub fn day1a(inputs: &[String]) -> String {
    // TODO: Handle errors, don't just unwrap and hope for the best
    let amounts: Vec<i32> = inputs.into_iter().map(|x| x.parse().unwrap()).collect();

    let mut lookup = HashSet::new();
    for amount in amounts.iter() {
        let desired = 2020 - amount;

        if lookup.contains(&desired) {
            return format!("{}", amount * desired);
        }

        lookup.insert(amount);
    }

    // TODO: Maybe error instead of returning a garbage value
    "error".to_string()
}

pub fn day1a_functional(inputs: &[String]) -> Option<String> {
    return inputs.
        into_iter().
        fold(Some(vec![]), |acc, s| match acc {
            Some(a) => {
                match s.parse::<i32>() {
                    Ok(i) => Some([a, vec![i]].concat()),
                    Err(_) => None
                }
            },
            None => None
        }).map(|amounts| {
            let mut lookup = HashSet::new();

            for amount in amounts.iter() {
                let desired = 2020 - amount;

                if lookup.contains(&desired) {
                    return Some(format!("{}", amount * desired));
                }

                lookup.insert(amount);
            }
            return None;
        }).flatten();
}

pub fn day1b(inputs: &[String]) -> String {
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
                return format!("{}", i * j * desired);
            }
        }
    }

    // TODO: Maybe error instead of returning a garbage value
    "error".to_string()
}
