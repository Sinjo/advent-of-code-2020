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
