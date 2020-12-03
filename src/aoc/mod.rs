use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::iter::FromIterator;

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

pub fn day3a(inputs: &[String]) -> anyhow::Result<String> {
    let width: u128 = inputs[0].chars().count().try_into().unwrap();
    let height: u128 = inputs.into_iter().count().try_into().unwrap();

    let tree_points: Vec<Vec<(u128, u128)>> = inputs.into_iter().enumerate().map(|(j, line)| {
        line.chars().enumerate().map(|(i, c)| {
            if c == '#' {
                return Some((u128::try_from(i).unwrap(),u128::try_from(j).unwrap()));
            } else {
                return None
            }
        }).filter_map(|p| p ).collect()
    }).collect();
    let flattened: Vec<(u128, u128)> = tree_points.into_iter().flatten().collect();

    let lookup: HashSet<(u128, u128)> = HashSet::from_iter(flattened);

    let mut x: u128 = 0;
    let mut trees = 0;
    for y in 0..height {
        if lookup.contains(&(x,y)) {
            trees += 1;
        }

        x = (x + 3) % width
    }

    Ok(trees.to_string())
}

pub fn day3b(inputs: &[String]) -> anyhow::Result<String> {
    let width: u128 = inputs[0].chars().count().try_into().unwrap();
    let height: u128 = inputs.into_iter().count().try_into().unwrap();

    let tree_points: Vec<Vec<(u128, u128)>> = inputs.into_iter().enumerate().map(|(j, line)| {
        line.chars().enumerate().map(|(i, c)| {
            if c == '#' {
                return Some((u128::try_from(i).unwrap(),u128::try_from(j).unwrap()));
            } else {
                return None
            }
        }).filter_map(|p| p ).collect()
    }).collect();
    let flattened: Vec<(u128, u128)> = tree_points.into_iter().flatten().collect();

    let lookup: HashSet<(u128, u128)> = HashSet::from_iter(flattened);

    // (right, down)
    let slopes = vec![
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];

    let multiplied_trees_per_slope = slopes.into_iter().map(|(right, down)| {
        let mut x = 0;
        let mut trees: u128 = 0;
        for y in 0..height {
            if y % down != 0 {
                continue;
            }

            if lookup.contains(&(x,y)) {
                trees += 1;
            }

            x = (x + right) % width
        }

        return trees;
    }).fold(1u128, |acc, trees| acc * trees);

    Ok(multiplied_trees_per_slope.to_string())
}
