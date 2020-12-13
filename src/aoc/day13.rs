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
