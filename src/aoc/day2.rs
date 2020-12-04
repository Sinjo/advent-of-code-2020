use regex::Regex;

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
