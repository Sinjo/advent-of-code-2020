use std::collections::HashSet;

// Remember to change window size for smaller input files
pub fn day9a(inputs: &[String]) -> anyhow::Result<String> {
    let numbers: Vec<usize> = inputs.iter().map(|i| i.parse().unwrap()).collect();

    let answer = numbers.windows(26).find_map(|slice| {
        if !can_sum_to_target(&slice[0..=24], slice[25]) {
            return Some(slice[25]);
        } else {
            return None;
        }
    }).unwrap();

    Ok(answer.to_string())
}

fn can_sum_to_target(inputs: &[usize], target: usize) -> bool {
    let mut lookup = HashSet::new();
    for input in inputs.iter() {
        lookup.insert(input);
    }

    for input in inputs.iter() {
        let desired = target - input;

        if lookup.contains(&desired) {
            return true;
        }
    }

    false
}
