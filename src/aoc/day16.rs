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

pub fn day16b(inputs: &[String]) -> anyhow::Result<String> {
    let rule_pattern = Regex::new(r"^(?P<rule_name>[\w ]+): (?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)$").unwrap();

    let parts: Vec<_> = inputs.split(|s| s.is_empty()).collect();
    let rules: Vec<_> = parts[0].to_vec();
    let our_ticket: Vec<_> = parts[1][1].split(",").map(|s| s.parse::<u128>().unwrap()).collect();
    let tickets = parts[2][1..parts[2].len()].to_vec();

    // need pairs of rule : name of rule
    let rule_tuples: Vec<_> = rules.iter().flat_map(|r| {
        let captures = rule_pattern.captures(r).unwrap();
        let rule_name = captures.name("rule_name").unwrap().as_str();
        let lower1: u128 = captures.name("lower1").unwrap().as_str().parse().unwrap();
        let upper1: u128 = captures.name("upper1").unwrap().as_str().parse().unwrap();
        let lower2: u128 = captures.name("lower2").unwrap().as_str().parse().unwrap();
        let upper2: u128 = captures.name("upper2").unwrap().as_str().parse().unwrap();

        vec![(rule_name, lower1, upper1, lower2, upper2)]
    }).collect();

    let parsed_tickets: Vec<_> = tickets.iter().map(|t| {
        t.split(",").map(|s| s.parse::<u128>().unwrap()).collect::<Vec<_>>()
    }).collect();

    let valid_tickets: Vec<Vec<_>> = parsed_tickets.iter().cloned().filter(|t| {
        !t.iter().cloned().any(|i| {
            !rule_tuples.iter().cloned().any(|(_, lower1, upper1, lower2, upper2)| {
                (i >= lower1 && i <= upper1) ||
                    (i >= lower2 && i <= upper2)
            })
        })
    }).collect();

    let columns: Vec<Vec<_>> = (0..valid_tickets[0].len()).map(|i| {
        valid_tickets.iter().map(|t| t[i]).collect()
    }).collect();

    let mut columns_to_potential_rules: Vec<(usize, Vec<String>)> = vec![];

    for column_id in 0..columns.len() {
        let column = &columns[column_id];

        let potential_rules: Vec<_> = rule_tuples.iter().cloned().filter_map(|rule| {
            match column.iter().all(|v| matches_rule(v, &rule)) {
                true => Some(rule.0.to_string()),
                false => None
            }
        }).collect();

        columns_to_potential_rules.push((column_id, potential_rules));
    }

    columns_to_potential_rules.sort_by(|a,b| a.1.len().cmp(&b.1.len()));

    let (column_ids, rule_names) = columns_to_potential_rules.iter().
        fold((vec![], vec![]), |(columns_seen, rules_seen), (column, potential_rules)| {
            let rule = potential_rules.iter().find(|r| !rules_seen.contains(r)).unwrap();

            let new_columns_seen = [columns_seen, vec![column]].concat();
            let new_rules_seen = [rules_seen, vec![rule]].concat();

            (new_columns_seen, new_rules_seen)
    });

    let columns_to_rules: Vec<_> = column_ids.iter().zip(rule_names.iter()).collect();
    let departure_columns: Vec<_> = columns_to_rules.iter().filter(|(_, r)| {
        r.starts_with(&"departure".to_string())
    }).collect();

    let answer: u128 = departure_columns.iter().fold(1, |acc, (index, _)| {
        acc * our_ticket[***index]
    });

    Ok(answer.to_string())
}

fn matches_rule(value: &u128, rule: &(&str, u128, u128, u128, u128)) -> bool {
    let (_, lower1, upper1, lower2, upper2) = rule;

    (value >= lower1 && value <= upper1) ||
        (value >= lower2 && value <= upper2)
}
