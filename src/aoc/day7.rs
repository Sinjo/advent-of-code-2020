use std::collections::HashMap;
use regex::Regex;

pub fn day7a(inputs: &[String]) -> anyhow::Result<String> {
    let rules = build_rules(inputs);

    let shiny_gold_containers: Vec<_> = rules.keys().filter_map(|container| {
        if eventually_contains_shiny_gold(&rules, &container) {
            return Some(container);
        } else {
            return None;
        }
    }).collect();
    let container_count = shiny_gold_containers.len();

    Ok(container_count.to_string())
}

pub fn day7b(inputs: &[String]) -> anyhow::Result<String> {
    let rules = build_rules(inputs);

    let bag_count = contained_bag_count(&rules, "shiny gold");

    Ok(bag_count.to_string())
}

fn eventually_contains_shiny_gold(rules: &HashMap<&str, Vec<(&str, usize)>>, container: &str) -> bool {
    let possible_contents = &rules[container];
    
    for (colour, _) in possible_contents.iter() {
        if colour == &"shiny gold" {
            return true;
        } else {
            let eventually_contained = eventually_contains_shiny_gold(rules, colour);
            if eventually_contained {
                return eventually_contained;
            }
        }
    }

    false
}

fn contained_bag_count(rules: &HashMap<&str, Vec<(&str, usize)>>, container: &str) -> usize {
    let possible_contents = &rules[container];
    let mut total: usize = 0;

    for (colour, count) in possible_contents.iter() {
        total += count;
        total += contained_bag_count(rules, colour) * count;
    }

    total
}

pub fn build_rules(inputs: &[String]) -> HashMap<&str, Vec<(&str, usize)>> {
    let initial_pattern = Regex::new(r"^(?P<container>(\w|\s)+) bags contain (?P<contents>(\w|\s|,)+)\.$").unwrap();
    let contents_pattern = Regex::new(r"(?P<count>\d+) (?P<colour>(\w|\s)+) bags?$").unwrap();

    let captured: Vec<_> = inputs.iter().map(|i| {
        initial_pattern.captures(i).unwrap()
    }).collect();

    let mut rules = HashMap::new();

    for rule in captured.iter() {
        let container = rule.name("container").unwrap().as_str();
        let unsplit_contents = rule.name("contents").unwrap().as_str();
        let split_contents: Vec<_> = unsplit_contents.split(", ").collect();

        let mut contents = Vec::new();

        for s in split_contents.iter() {
            if !(s == &"no other bags") {
                let parsed_contents = contents_pattern.captures(s).unwrap();
                let count: usize = parsed_contents.name("count").unwrap().as_str().parse().unwrap();
                let colour = parsed_contents.name("colour").unwrap().as_str();

                contents.push((colour, count));
            }

        };

        rules.insert(container, contents);
    }

    rules
}
