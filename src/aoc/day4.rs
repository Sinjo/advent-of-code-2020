use regex::Regex;
use std::collections::HashMap;

pub fn day4a(inputs: &[String]) -> anyhow::Result<String> {
    let pattern = Regex::new(r"(\w+):((\w|#)+)").unwrap();

    let records: Vec<String> = inputs.split(|s| s.is_empty()).
        map(|slice| slice.into_iter().cloned().collect::<Vec<_>>().join(" ")).collect();

    let mut parsed: Vec<HashMap<String,String>> = records.into_iter().map(|r| {
        let captures: Vec<_> = pattern.find_iter(&r).map(|pair| {
            let split: Vec<_> = pair.as_str().split(":").collect();
            let k = split[0].to_string();
            let v = split[1].to_string();

            (k, v)
        }).collect();

        let mut capture_hash: HashMap<String, String> = HashMap::new();
        for (k,v) in captures.iter() {
            capture_hash.insert(k.to_string(), v.to_string());
        }
        capture_hash
    }).collect();

    for p in parsed.iter_mut() {
        p.remove("cid");
    }

    Ok(parsed.into_iter().filter(|r| validate_record(r, true)).count().to_string())
}

pub fn day4b(inputs: &[String]) -> anyhow::Result<String> {
    let pattern = Regex::new(r"(\w+):((\w|#)+)").unwrap();

    let records: Vec<String> = inputs.split(|s| s.is_empty()).
        map(|slice| slice.into_iter().cloned().collect::<Vec<_>>().join(" ")).collect();

    let mut parsed: Vec<HashMap<String,String>> = records.into_iter().map(|r| {
        let captures: Vec<_> = pattern.find_iter(&r).map(|pair| {
            let split: Vec<_> = pair.as_str().split(":").collect();
            let k = split[0].to_string();
            let v = split[1].to_string();

            (k, v)
        }).collect();

        let mut capture_hash: HashMap<String, String> = HashMap::new();
        for (k,v) in captures.iter() {
            capture_hash.insert(k.to_string(), v.to_string());
        }
        capture_hash
    }).collect();

    for p in parsed.iter_mut() {
        p.remove("cid");
    }

    Ok(parsed.into_iter().filter(|r| validate_record(r, false)).count().to_string())
}

pub fn validate_record(record: &HashMap<String,String>, ignore_values: bool) -> bool {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let all_keys_present = keys.into_iter().
        map(|k| record.contains_key(k)).
        fold(true, |acc, key_present| acc & key_present);

    if ignore_values {
        return all_keys_present;
    }

    if !all_keys_present {
        return false;
    }

    // println!("{}", validate_byr(&record["byr"]));
    // println!("{}", validate_iyr(&record["iyr"]));
    // println!("{}", validate_eyr(&record["eyr"]));
    // println!("{}", validate_hgt(&record["hgt"]));
    // println!("{}", validate_hcl(&record["hcl"]));
    // println!("{}", validate_ecl(&record["ecl"]));
    // println!("{}", validate_pid(&record["pid"]));
    // println!("=====");

    validate_byr(&record["byr"]) &&
        validate_iyr(&record["iyr"]) &&
        validate_eyr(&record["eyr"]) &&
        validate_hgt(&record["hgt"]) &&
        validate_hcl(&record["hcl"]) &&
        validate_ecl(&record["ecl"]) &&
        validate_pid(&record["pid"])
}

pub fn validate_byr(value: &str) -> bool {
    let year: usize = match value.parse::<usize>() {
        Ok(y) => y,
        Err(_) => return false
    };

    return year >= 1920 && year <= 2002
}

pub fn validate_iyr(value: &str) -> bool {
    let year: usize = match value.parse::<usize>() {
        Ok(y) => y,
        Err(_) => return false
    };

    return year >= 2010 && year <= 2020
}

pub fn validate_eyr(value: &str) -> bool {
    let year: usize = match value.parse::<usize>() {
        Ok(y) => y,
        Err(_) => return false
    };

    return year >= 2020 && year <= 2030
}

pub fn validate_hgt(value: &str) -> bool {
    let in_pattern = Regex::new(r"^(\d+)in$").unwrap();
    match in_pattern.captures(value) {
        Some(c) => match c.get(1) {
            Some(s) => match s.as_str().parse::<usize>() {
                Ok(i) => return i >= 59 && i <=76,
                Err(_) => ()
            },
            None => ()
        }
        None => ()
    }


    let cm_pattern = Regex::new(r"(\d+)cm").unwrap();
    match cm_pattern.captures(value) {
        Some(c) => match c.get(1) {
            Some(s) => match s.as_str().parse::<usize>() {
                Ok(i) => return i >= 150 && i <= 193,
                Err(_) => ()
            },
            None => ()
        }
        None => ()
    }

    false
}

pub fn validate_hcl(value: &str) -> bool {
    let pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return pattern.is_match(value)
}

pub fn validate_ecl(value: &str) -> bool {
    let eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return eye_colours.contains(&value)
}

pub fn validate_pid(value: &str) -> bool {
    let pattern = Regex::new(r"^\d{9}$").unwrap();
    return pattern.is_match(value)
}
