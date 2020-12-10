pub fn day10a(inputs: &[String]) -> anyhow::Result<String> {
    let mut adapters: Vec<usize> = inputs.iter().map(|i| i.parse().unwrap()).collect();
    adapters.sort();

    let mut last_joltage = 0;
    let mut one_jolt_difference = 0;
    let mut three_jolt_difference = 0;

    for joltage in adapters.iter() {
        let difference = joltage - last_joltage;

        if difference == 1 {
            one_jolt_difference += 1;
        } else if difference == 3 {
            three_jolt_difference += 1;
        }

        last_joltage = *joltage;
    }

    // Our adapter is always 3 higher than the highest adapter
    three_jolt_difference += 1;

    let answer = one_jolt_difference * three_jolt_difference;

    Ok(answer.to_string())
}

pub fn day10b(inputs: &[String]) -> anyhow::Result<String> {
    let mut adapters: Vec<usize> = inputs.iter().map(|i| i.parse().unwrap()).collect();
    let device_jolts = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(device_jolts);
    adapters.sort();

    let answer_length = adapters.len();
    let mut answer: Vec<u128> = vec![0; answer_length];

    answer[answer_length - 1] = 1;
    answer[answer_length - 2] = 1;
    answer[answer_length - 3] = 1;

    let start_position = answer_length - 3;
    for i in (0..start_position).rev() {
        // Jump to next adapter, always possible
        answer[i] += answer[i+1];
        // Jump to two after
        if adapters[i+2] - adapters[i] <= 3 {
            answer[i] += answer[i+2];
        }
        // Jump to three after
        if adapters[i+3] - adapters[i] <= 3 {
            answer[i] += answer[i+3];
        }
    }

    Ok(answer[0].to_string())
}
