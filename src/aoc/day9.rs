use std::collections::HashSet;

use anyhow::anyhow;

// Remember to change window size for smaller input files
pub fn day9a(inputs: &[String]) -> anyhow::Result<String> {
    let numbers: Vec<usize> = inputs.iter().map(|i| i.parse().unwrap()).collect();

    let answer = find_non_sum_of_previous(&numbers, 25);

    Ok(answer.to_string())
}

pub fn day9b(inputs: &[String]) -> anyhow::Result<String> {
    let numbers: Vec<usize> = inputs.iter().map(|i| i.parse().unwrap()).collect();
    let target = find_non_sum_of_previous(&numbers, 25);

    // Approach
    //   - For each position in numbers array
    //     - start summing from there, aiming for the target
    //     - if we overshoot, break and start from the next number
    //     - if we reach the target
    //       - get the number from the start position, sum with current number, return
    for (start, _) in numbers.iter().enumerate() {
        let mut sum = 0;
        for potential_end in start..numbers.len() {
            // println!("start: {}, potential_end: {}", start, potential_end)
            sum += numbers[potential_end];
            if sum == target {
                return Ok((numbers[start] + numbers[potential_end]).to_string());
            } else if sum > target {
                break;
            }
        }
    }

    Err(anyhow!("didn't find answer"))
}

fn find_non_sum_of_previous(inputs: &[usize], window_size: usize) -> usize {
    inputs.windows(window_size + 1).find_map(|slice| {
        if !can_sum_to_target(&slice[0..window_size], slice[window_size]) {
            return Some(slice[25]);
        } else {
            return None;
        }
    }).unwrap()
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
