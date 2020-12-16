use regex::Regex;

pub fn day16a(inputs: &[String]) -> anyhow::Result<String> {
    let rule_pattern = Regex::new(r"^(?P<rule_name>[\w ]+): (?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)$").unwrap();

    let parts: Vec<_> = inputs.split(|s| s.is_empty()).collect();
    let rules: Vec<_> = parts[0].to_vec();
    let tickets = parts[2][1..parts[2].len()].to_vec();

    let rule_tuples: Vec<_> = rules.iter().flat_map(|r| {
        let captures = rule_pattern.captures(r).unwrap();
        let lower1: u128 = captures.name("lower1").unwrap().as_str().parse().unwrap();
        let upper1: u128 = captures.name("upper1").unwrap().as_str().parse().unwrap();
        let lower2: u128 = captures.name("lower2").unwrap().as_str().parse().unwrap();
        let upper2: u128 = captures.name("upper2").unwrap().as_str().parse().unwrap();

        vec![(lower1, upper1), (lower2, upper2)]
    }).collect();

    let parsed_tickets: Vec<_> = tickets.iter().map(|t| {
        t.split(",").map(|s| s.parse::<u128>().unwrap()).collect::<Vec<_>>()
    }).collect();

    let invalid_values: Vec<u128> = parsed_tickets.iter().flat_map(|t| {
        t.iter().cloned().filter(|i| {
            !rule_tuples.iter().any(|(lower, upper)| {
                i >= &lower && i <= &upper
            })
        }).collect::<Vec<_>>()
    }).collect();

    let answer: u128 = invalid_values.iter().sum();

    Ok(answer.to_string())
}
