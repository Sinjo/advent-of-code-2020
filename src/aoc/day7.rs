use std::collections::HashMap;
use regex::Regex;

pub fn day7a(inputs: &[String]) -> anyhow::Result<String> {
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

        for s in split_contents.iter() {
            if s == &"no other bags" {
                rules.insert(container, None);
            } else {
                let parsed_contents = contents_pattern.captures(s).unwrap();
                let count: usize = parsed_contents.name("count").unwrap().as_str().parse().unwrap();
                let colour = parsed_contents.name("colour").unwrap().as_str();

                rules.insert(container, Some((colour, count)));
            }

        };
    }

    println!("{:#?}", rules);

    Ok("whatever".to_string())
}
