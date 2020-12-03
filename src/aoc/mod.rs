use std::collections::HashSet;

use regex::Regex;

pub fn do_aoc_stuff() -> bool {
    true
}

pub fn day1a(inputs: &[String]) -> anyhow::Result<String> {
    // TODO: Handle errors, don't just unwrap and hope for the best
    let amounts: Vec<i32> = inputs.into_iter().map(|x| x.parse().unwrap()).collect();

    let mut lookup = HashSet::new();
    for amount in amounts.iter() {
        let desired = 2020 - amount;

        if lookup.contains(&desired) {
            return Ok(format!("{}", amount * desired));
        }

        lookup.insert(amount);
    }

    // TODO: Maybe error instead of returning a garbage value
    Ok("error".to_string())
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

pub fn day1b(inputs: &[String]) -> anyhow::Result<String> {
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
                return Ok(format!("{}", i * j * desired));
            }
        }
    }

    // TODO: Maybe error instead of returning a garbage value
    Ok("error".to_string())
}

pub fn day2a(inputs: &[String]) -> anyhow::Result<String> {
    let pattern = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>[a-z]):\s(?P<password>\w+)").unwrap();

    let parsed: Vec<Option<regex::Captures>> = inputs.into_iter().map(|s| pattern.captures(s)).collect();

    let validity: Vec<bool> = parsed.into_iter().map(|captures| match captures {
        Some(c) => {
            let lower: usize = c.name("lower").unwrap().as_str().parse().unwrap();
            let upper: usize = c.name("upper").unwrap().as_str().parse().unwrap();
            let letter = c.name("letter").unwrap().as_str();
            let password = c.name("password").unwrap().as_str();

            let occurrences = password.matches(letter).count();

            return occurrences >= lower && occurrences <= upper;
        },
        None => false
    }).collect();

    let valid_count = validity.into_iter().filter(|b| *b).count();

    return Ok(valid_count.to_string());
}

pub fn day2b(inputs: &[String]) -> anyhow::Result<String> {
    let pattern = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>[a-z]):\s(?P<password>\w+)").unwrap();

    let parsed: Vec<Option<regex::Captures>> = inputs.into_iter().map(|s| pattern.captures(s)).collect();

    let validity: Vec<bool> = parsed.into_iter().map(|captures| match captures {
        Some(c) => {
            let lower: usize = c.name("lower").unwrap().as_str().parse().unwrap();
            let upper: usize = c.name("upper").unwrap().as_str().parse().unwrap();
            let letter = c.name("letter").unwrap().as_str();
            let password = c.name("password").unwrap().as_str();

            let letter_char = letter.chars().next().unwrap();
            let password_chars: Vec<char> = password.chars().collect();

            let lower_matches = password_chars[lower-1usize] == letter_char;
            let upper_matches = password_chars[upper-1usize] == letter_char;

            return lower_matches ^ upper_matches
        },
        None => false
    }).collect();

    let valid_count = validity.into_iter().filter(|b| *b).count();

    return Ok(valid_count.to_string());
}
